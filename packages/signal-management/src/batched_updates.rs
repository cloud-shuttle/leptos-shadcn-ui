//! Batched signal updates for better performance

use leptos::prelude::*;

use crate::error::SignalManagementError;

/// Batched signal updates for better performance
/// 
/// This struct provides a mechanism to batch multiple signal updates
/// together, reducing the number of reactive updates and improving performance.
#[derive(Debug, Clone)]
pub struct BatchedSignalUpdater {
    /// Queue of updates to be executed
    pub update_queue: ArcRwSignal<Vec<Box<dyn Fn() + Send + Sync>>>,
    /// Whether currently batching updates
    is_batching: ArcRwSignal<bool>,
    /// Maximum number of updates to batch
    pub max_batch_size: usize,
}

impl BatchedSignalUpdater {
    /// Create a new batched signal updater
    pub fn new() -> Self {
        Self {
            update_queue: ArcRwSignal::new(Vec::new()),
            is_batching: ArcRwSignal::new(false),
            max_batch_size: 1000, // Default maximum batch size
        }
    }
    
    /// Create a new batched signal updater with custom batch size
    pub fn with_batch_size(max_batch_size: usize) -> Self {
        Self {
            update_queue: ArcRwSignal::new(Vec::new()),
            is_batching: ArcRwSignal::new(false),
            max_batch_size,
        }
    }
    
    /// Queue an update for batched execution
    pub fn queue_update<F>(&self, update: F) -> Result<(), SignalManagementError>
    where
        F: Fn() + Send + Sync + 'static,
    {
        // Check if we're at the maximum batch size
        let current_size = self.update_queue.with(|queue| queue.len());
        if current_size >= self.max_batch_size {
            return Err(SignalManagementError::batched_update_failed(
                format!("Maximum batch size of {} exceeded", self.max_batch_size),
            ));
        }
        
        self.update_queue.update(|queue| {
            queue.push(Box::new(update));
        });
        
        Ok(())
    }
    
    /// Flush all queued updates
    pub fn flush_updates(&self) -> Result<(), SignalManagementError> {
        let mut updates = Vec::new();
        self.update_queue.update(|queue| {
            std::mem::swap(queue, &mut updates);
        });
        
        // Execute all updates
        for update in updates {
            update();
        }
        
        Ok(())
    }
    
    /// Start batching updates
    pub fn start_batching(&self) {
        self.is_batching.set(true);
    }
    
    /// End batching and flush updates
    pub fn end_batching(&self) -> Result<(), SignalManagementError> {
        self.is_batching.set(false);
        self.flush_updates()
    }
    
    /// Check if currently batching
    pub fn is_batching(&self) -> bool {
        self.is_batching.get()
    }
    
    /// Get the current queue size
    pub fn queue_size(&self) -> usize {
        self.update_queue.with(|queue| queue.len())
    }
    
    /// Get the maximum batch size
    pub fn max_batch_size(&self) -> usize {
        self.max_batch_size
    }
    
    /// Clear the update queue without executing updates
    pub fn clear_queue(&self) {
        self.update_queue.set(Vec::new());
    }
    
    /// Execute updates in batches of specified size
    pub fn flush_in_batches(&self, batch_size: usize) -> Result<(), SignalManagementError> {
        let mut updates = Vec::new();
        self.update_queue.update(|queue| {
            std::mem::swap(queue, &mut updates);
        });
        
        while !updates.is_empty() {
            let batch: Vec<_> = updates.drain(0..batch_size.min(updates.len())).collect();
            
            // Execute batch
            for update in batch {
                update();
            }
        }
        
        Ok(())
    }
    
    /// Clear all queued updates
    pub fn clear_updates(&self) -> Result<(), SignalManagementError> {
        self.update_queue.update(|queue| {
            queue.clear();
        });
        Ok(())
    }
    
    /// Stop batching mode
    pub fn stop_batching(&self) -> Result<(), SignalManagementError> {
        self.is_batching.set(false);
        Ok(())
    }
}

impl Default for BatchedSignalUpdater {
    fn default() -> Self {
        Self::new()
    }
}

/// Utility for managing multiple batched updaters
pub struct BatchedUpdaterManager {
    updaters: Vec<BatchedSignalUpdater>,
}

impl BatchedUpdaterManager {
    /// Create a new batched updater manager
    pub fn new() -> Self {
        Self {
            updaters: Vec::new(),
        }
    }
    
    /// Add a new batched updater
    pub fn add_updater(&mut self, updater: BatchedSignalUpdater) {
        self.updaters.push(updater);
    }
    
    /// Get the number of updaters
    pub fn updater_count(&self) -> usize {
        self.updaters.len()
    }
    
    /// Flush all updaters
    pub fn flush_all(&self) -> Result<(), SignalManagementError> {
        for updater in &self.updaters {
            updater.flush_updates()?;
        }
        Ok(())
    }
    
    /// Start batching on all updaters
    pub fn start_batching_all(&self) {
        for updater in &self.updaters {
            updater.start_batching();
        }
    }
    
    /// End batching on all updaters
    pub fn end_batching_all(&self) -> Result<(), SignalManagementError> {
        for updater in &self.updaters {
            updater.end_batching()?;
        }
        Ok(())
    }
    
    /// Get total queue size across all updaters
    pub fn total_queue_size(&self) -> usize {
        self.updaters.iter().map(|u| u.queue_size()).sum()
    }
}

impl Default for BatchedUpdaterManager {
    fn default() -> Self {
        Self::new()
    }
}
