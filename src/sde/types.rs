pub(crate) mod ids {
    //STATIC DATA
    pub(crate) type NameID = u32;
    pub(crate) type DescriptionID = u32;

    // MAP
    pub(crate) type GalaxyID = u32;
    pub(crate) type RegionID = u32;
    pub(crate) type ConstellationID = u32;
    pub(crate) type SolarSystemID = u32;
    pub(crate) type StargateID = u32;
    pub(crate) type PositionID = u32;
    pub(crate) type Position2dID = u32;

    // ENTITY
    pub(crate) type TypeID = u32;
    pub(crate) type CategoryID = u32;
    pub(crate) type GroupID = u32;

    // INDUSTRY
    pub(crate) type IndustryActivityID = u32;
}

pub(crate) mod values {
    pub(crate) type BuildNumber = u32;
    pub(crate) type Volume = f64;
    pub(crate) type SecurityStatus = f64;
    pub(crate) type Coordinate = f64;
    pub(crate) type MaxProductionLimit = u32;
    pub(crate) type Time = u32;
    pub(crate) type Quantity = u32;
    pub(crate) type Probability = f64;
    pub(crate) type Level = u8;
}
