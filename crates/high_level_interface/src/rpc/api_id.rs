use serde::Serialize;

#[allow(unused)]
#[derive(Copy, Clone, Debug)]
pub enum ApiId {
    ChangeMode = 2000,
    Move = 2001,
    RotateHead = 2004,
    WaveHand = 2005,
    RotateHeadWithDirection = 2006,
    LieDown = 2007,
    GetUp = 2008,
    MoveHandEndEffector = 2009,
    ControlGripper = 2010,
    GetFrameTransform = 2011,
    SwitchHandEndEffectorControlMode = 2012,
    ControlDexterousHand = 2013,
    Handshake = 2015,
    Dance = 2016,
    GetMode = 2017,
    GetStatus = 2018,
    PushUp = 2019,
    PlaySound = 2020,
    StopSound = 2021,
    GetRobotInfo = 2022,
    StopHandEndEffector = 2023,
    Shoot = 2024,
    GetUpWithMode = 2025,
    ZeroTorqueDrag = 2026,
    RecordTrajectory = 2027,
    ReplayTrajectory = 2028,
    WholeBodyDance = 2029,
    UpperBodyCustomControl = 2030,
    ResetOdometry = 2031,
    LoadCustomTrainedTraj = 2032,
    ActivateCustomTrainedTraj = 2033,
    UnloadCustomTrainedTraj = 2034,
    EnterWBCGait = 2035,
    ExitWBCGait = 2036,
}

impl Serialize for ApiId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u64(*self as u64)
    }
}
