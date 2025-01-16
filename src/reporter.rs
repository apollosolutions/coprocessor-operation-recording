use std::{collections::HashMap, fs::OpenOptions, sync::Arc};
use chrono::{DateTime, Utc};
use tokio::sync::RwLock;
use tracing::{debug, error};

use crate::{config, handler};

#[derive( Debug, Clone)]
pub struct Reporter {
    config: config::Config,
    pub reports: Arc<RwLock<HashMap<String, Report>>>,
}

#[derive(Debug, Clone)]
pub struct Report {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub operation_name: String,
    pub client_name: String,
    pub client_version: String,
    pub status_code: u16,
}

impl Reporter {
    pub fn new(config: config::Config) -> Self {
        let reporter = Self {
            config,
            reports: Arc::new(RwLock::new(HashMap::new())),
        };
        tokio::task::spawn(poll(reporter.clone()));
        reporter
    }

    pub async fn add_report(&self, request: handler::CoprocessorRequest) {
        let mut reports = self.reports.write().await;
        
        let (client_name, client_version, operation_name)  = match request.context {
            Some(context) => {
                let client_name = context.entries.get("apollo_telemetry::client_name").map(|v| v.as_str().unwrap_or("Unknown").to_string());
                let client_version = context.entries.get("apollo_telemetry::client_version").map(|v| v.as_str().unwrap_or("Unknown").to_string());
                let operation_name = context.entries.get("operation_name").map(|v| v.as_str().unwrap_or("Anonymous Operation").to_string());
                
                (client_name, client_version, operation_name)
            }
            None => (None, None, None),
        };
        
        let report = Report {
            id: request.id.clone(),
            timestamp: Utc::now(),
            operation_name: operation_name.unwrap_or("Anonymous Operation".to_string()),
            client_name: client_name.unwrap_or("Unknown".to_string()),
            client_version: client_version.unwrap_or("Unknown".to_string()),
            status_code: request.status_code.unwrap_or(0),
        };
        debug!("Adding report: {:?}", report);
        reports.insert(report.id.clone(), report);

        // batch size 0 means we'll write the report during the poll, no time else
        if self.config.batch_size != 0 && reports.len() >= usize::try_from(self.config.batch_size).unwrap_or(0) {
            debug!("Batch limit reached, processing reports");
            drop(reports);
            let res = self.process_records().await;
            match res {
                Ok(_) => {
                    debug!("Processed reports successfully");
                }
                Err(e) => {
                    error!("Error processing reports: {:?}", e);
                }
            }
        }
    }

    pub async fn process_records(&self) -> std::io::Result<()>{
        // we've now claimed the reports for now to write them
        let mut reports = self.reports.write().await;
        debug!("Processing {} reports", reports.len());
        
        // If there are no reports, return early
        if reports.len()==0 {
            return Ok(());
        }

        let current_file = Utc::now().format("%Y-%m-%d-requests.csv").to_string();

        let file = OpenOptions::new()
            .read(true)
            
            .create(true)
            .append(true)
            .open(current_file.clone())?;

        let mut wtr = csv::Writer::from_writer(file.try_clone()?);
        
        {
            // Pushing the reader in the closure since it's only needed in this scope
            let mut rdr = csv::ReaderBuilder::new()
                .has_headers(true)
                .from_reader(file.try_clone()?);

            // Check for headers, and if they don't exist, write them
            if rdr.headers()?.is_empty() {
                wtr.write_record(["id", "timestamp", "operation_name", "client_name", "client_version", "status_code"])?;
            }
        }

        for (id, report) in reports.iter() {
            debug!("Processing report: {} {:?}", id, report);
            if report.id.is_empty() {
                continue;
            }
            wtr.serialize((
                report.id.clone(),
                report.timestamp.to_rfc3339(),
                report.operation_name.clone(),
                report.client_name.clone(),
                report.client_version.clone(),
                report.status_code,
            ))?;
        }
        reports.clear();
        Ok(())
    }
}

pub async fn poll (reporter: Reporter) {
    let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(reporter.config.interval));
    loop {
        interval.tick().await;
        debug!("Processing reports...");
        let res = reporter.process_records().await;
        match res {
            Ok(_) => {
                debug!("Processed reports successfully");
            }
            Err(e) => {
                error!("Error processing reports: {:?}", e);
            }
        }
    }
}
