//! CanBusObserver wrapper for mdflib CanBusObserver
//!
//! A CAN bus observer is used to read CAN messages from an MDF file. It provides
//! access to CAN messages parsed from the channel data.

use crate::canmessage::CanMessageRef;
use crate::error::Result;
use mdflib_sys as ffi;
use std::marker::PhantomData;

/// Represents an immutable reference to a CAN bus observer in an MDF file.
///
/// A CAN bus observer holds CAN message data for a specific channel group that
/// contains CAN bus data and provides methods to access CAN messages.
#[derive(Debug, Clone, Copy)]
pub struct CanBusObserverRef<'a> {
    pub(crate) inner: *const ffi::CanBusObserver,
    _marker: PhantomData<&'a ()>,
}

impl std::fmt::Display for CanBusObserverRef<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "CanBusObserver {{ name: '{}', nof_samples: {} }}",
            self.get_name(),
            self.get_nof_samples()
        )
    }
}

impl<'a> CanBusObserverRef<'a> {
    #[allow(dead_code)]
    pub(crate) fn new(inner: *const ffi::CanBusObserver) -> Self {
        Self {
            inner,
            _marker: PhantomData,
        }
    }

    /// Gets the name of this CAN bus observer.
    pub fn get_name(&self) -> String {
        let mut buffer = vec![0u8; 256];
        let len = unsafe {
            ffi::CanBusObserverGetName(
                self.inner as *mut ffi::CanBusObserver,
                buffer.as_mut_ptr() as *mut u8,
                buffer.len(),
            )
        };
        if len > 0 && len < buffer.len() {
            buffer.truncate(len);
            String::from_utf8_lossy(&buffer).to_string()
        } else {
            String::new()
        }
    }

    /// Gets the number of CAN messages (samples) in this observer.
    pub fn get_nof_samples(&self) -> usize {
        unsafe { ffi::CanBusObserverGetNofSamples(self.inner) }
    }

    /// Gets the CAN message for a specific sample.
    ///
    /// # Arguments
    /// * `sample` - The sample index (0-based)
    ///
    /// # Returns
    /// Returns `Some(CanMessageRef)` if the sample is valid and contains a CAN message,
    /// `None` otherwise.
    pub fn get_can_message(&self, sample: usize) -> Option<CanMessageRef<'a>> {
        let can_msg_ptr = unsafe {
            ffi::CanBusObserverGetCanMessage(self.inner as *mut ffi::CanBusObserver, sample)
        };
        if can_msg_ptr.is_null() {
            None
        } else {
            Some(CanMessageRef::new(can_msg_ptr))
        }
    }

    /// Gets all CAN messages for all samples.
    ///
    /// # Returns
    /// Returns a vector of `Option<CanMessageRef>` where `None` indicates a sample
    /// without a valid CAN message.
    pub fn get_all_can_messages(&self) -> Vec<Option<CanMessageRef<'a>>> {
        let nof_samples = self.get_nof_samples();
        let mut messages = Vec::with_capacity(nof_samples);
        for sample in 0..nof_samples {
            messages.push(self.get_can_message(sample));
        }
        messages
    }
}

/// Represents a mutable CAN bus observer in an MDF file.
///
/// This wrapper provides ownership of the underlying CanBusObserver and automatically
/// cleans up resources when dropped.
#[derive(Debug)]
pub struct CanBusObserver<'a> {
    pub(crate) inner: *mut ffi::CanBusObserver,
    _marker: PhantomData<&'a ()>,
}

impl<'a> CanBusObserver<'a> {
    #[allow(dead_code)]
    pub(crate) fn new(inner: *mut ffi::CanBusObserver) -> Self {
        Self {
            inner,
            _marker: PhantomData,
        }
    }
}

// Implement Deref to allow using CanBusObserver as CanBusObserverRef
impl<'a> std::ops::Deref for CanBusObserver<'a> {
    type Target = CanBusObserverRef<'a>;

    fn deref(&self) -> &Self::Target {
        unsafe { &*(self as *const CanBusObserver as *const CanBusObserverRef) }
    }
}

// Implement Drop to clean up the observer when it goes out of scope
impl<'a> Drop for CanBusObserver<'a> {
    fn drop(&mut self) {
        if !self.inner.is_null() {
            unsafe {
                ffi::CanBusObserverUnInit(self.inner);
            }
        }
    }
}

// Safety: CanBusObserver can be safely sent between threads
unsafe impl<'a> Send for CanBusObserver<'a> {}
unsafe impl<'a> Sync for CanBusObserver<'a> {}

/// Creates a CAN bus observer for a specific channel group in a data group.
///
/// This function creates a CAN bus observer that can be used to read CAN message data
/// from a channel group that contains CAN bus data. The observer automatically parses
/// the CAN messages from the underlying channel data.
///
/// # Safety
///
/// This function is unsafe because it takes raw pointers to C++ objects. The caller
/// must ensure that:
/// - The pointers are valid and point to live objects
/// - The objects remain valid for the lifetime of the observer
/// - The pointers are properly aligned and non-null
/// - The channel group contains CAN bus data
///
/// # Arguments
/// * `data_group` - Raw pointer to the data group containing the channel group
/// * `channel_group` - Raw pointer to the channel group containing CAN data
///
/// # Returns
/// Returns a `Result<CanBusObserver>` if successful, or an error if creation fails.
///
/// # Example
/// ```no_run
/// use mdflib::*;
///
/// # fn example() -> mdflib::Result<()> {
/// let reader = reader::MdfReader::new("can_data.mf4")?;
/// // ... get data_group and channel_group from file ...
/// # let file = reader.get_file().unwrap();
/// # let data_group = file.get_data_group(0).unwrap();
/// # let channel_group = data_group.get_channel_group_by_index(0).unwrap();
///
/// // Only create CAN bus observer for CAN channel groups
/// if channel_group.get_bus_type() == BusType::Can as u8 {
///     let observer = unsafe {
///         create_can_bus_observer(data_group.as_ptr(), channel_group.as_ptr())?
///     };
///     let nof_samples = observer.get_nof_samples();
///
///     for sample in 0..nof_samples {
///         if let Some(can_msg) = observer.get_can_message(sample) {
///             println!("CAN message {}: ID=0x{:X}, DLC={}",
///                     sample, can_msg.get_can_id(), can_msg.get_dlc());
///         }
///     }
/// }
/// # Ok(())
/// # }
/// ```
pub unsafe fn create_can_bus_observer<'a>(
    data_group: *const ffi::IDataGroup,
    channel_group: *const ffi::IChannelGroup,
) -> Result<CanBusObserver<'a>> {
    let observer = unsafe { ffi::CreateCanBusObserver(data_group, channel_group) };

    if observer.is_null() {
        return Err(crate::error::MdfError::NullPointer);
    }

    Ok(CanBusObserver::new(observer))
}
