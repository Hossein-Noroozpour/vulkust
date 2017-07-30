pub mod manager;

pub trait Audio {}

pub struct Music {}

impl Audio for Music {}

pub struct Voice {}

impl Audio for Voice {}