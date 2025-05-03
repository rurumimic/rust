pub mod seoul {
    include!("pb/city.seoul.rs");
}

pub mod newyork {
    include!("pb/city.newyork.rs");
}

pub mod tokyo {
    include!("pb/city.tokyo.rs");
}

pub mod reflection {
    pub const FILE_DESCRIPTOR_SET: &[u8] = tonic::include_file_descriptor_set!("descriptor");
}
