#[doc = "Register `STS2` reader"]
pub type R = crate::R<STS2_SPEC>;
#[doc = "Transmission mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRMODE_A {
    #[doc = "0: Slave mode"]
    Slave = 0,
    #[doc = "1: Master mode"]
    Master = 1,
}
impl From<TRMODE_A> for bool {
    #[inline(always)]
    fn from(variant: TRMODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRMODE` reader - Transmission mode"]
pub type TRMODE_R = crate::BitReader<TRMODE_A>;
impl TRMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TRMODE_A {
        match self.bits {
            false => TRMODE_A::Slave,
            true => TRMODE_A::Master,
        }
    }
    #[doc = "Slave mode"]
    #[inline(always)]
    pub fn is_slave(&self) -> bool {
        *self == TRMODE_A::Slave
    }
    #[doc = "Master mode"]
    #[inline(always)]
    pub fn is_master(&self) -> bool {
        *self == TRMODE_A::Master
    }
}
#[doc = "Bus busy\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUSYF_A {
    #[doc = "0: Bus idle"]
    Idle = 0,
    #[doc = "1: Bus busy"]
    Busy = 1,
}
impl From<BUSYF_A> for bool {
    #[inline(always)]
    fn from(variant: BUSYF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUSYF` reader - Bus busy"]
pub type BUSYF_R = crate::BitReader<BUSYF_A>;
impl BUSYF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BUSYF_A {
        match self.bits {
            false => BUSYF_A::Idle,
            true => BUSYF_A::Busy,
        }
    }
    #[doc = "Bus idle"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == BUSYF_A::Idle
    }
    #[doc = "Bus busy"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == BUSYF_A::Busy
    }
}
#[doc = "Transmission direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRF_A {
    #[doc = "0: Data reception"]
    Reception = 0,
    #[doc = "1: Data transmission"]
    Transmission = 1,
}
impl From<DIRF_A> for bool {
    #[inline(always)]
    fn from(variant: DIRF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIRF` reader - Transmission direction"]
pub type DIRF_R = crate::BitReader<DIRF_A>;
impl DIRF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DIRF_A {
        match self.bits {
            false => DIRF_A::Reception,
            true => DIRF_A::Transmission,
        }
    }
    #[doc = "Data reception"]
    #[inline(always)]
    pub fn is_reception(&self) -> bool {
        *self == DIRF_A::Reception
    }
    #[doc = "Data transmission"]
    #[inline(always)]
    pub fn is_transmission(&self) -> bool {
        *self == DIRF_A::Transmission
    }
}
#[doc = "General call address reception\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GCADDRF_A {
    #[doc = "0: General call address is not received"]
    NotReceived = 0,
    #[doc = "1: General call address is received"]
    Received = 1,
}
impl From<GCADDRF_A> for bool {
    #[inline(always)]
    fn from(variant: GCADDRF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GCADDRF` reader - General call address reception"]
pub type GCADDRF_R = crate::BitReader<GCADDRF_A>;
impl GCADDRF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GCADDRF_A {
        match self.bits {
            false => GCADDRF_A::NotReceived,
            true => GCADDRF_A::Received,
        }
    }
    #[doc = "General call address is not received"]
    #[inline(always)]
    pub fn is_not_received(&self) -> bool {
        *self == GCADDRF_A::NotReceived
    }
    #[doc = "General call address is received"]
    #[inline(always)]
    pub fn is_received(&self) -> bool {
        *self == GCADDRF_A::Received
    }
}
#[doc = "SMBus device address receiving\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DEVADDRF_A {
    #[doc = "0: SMBus device address is not received"]
    NotReceived = 0,
    #[doc = "1: SMBus device address is received"]
    Received = 1,
}
impl From<DEVADDRF_A> for bool {
    #[inline(always)]
    fn from(variant: DEVADDRF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DEVADDRF` reader - SMBus device address receiving"]
pub type DEVADDRF_R = crate::BitReader<DEVADDRF_A>;
impl DEVADDRF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DEVADDRF_A {
        match self.bits {
            false => DEVADDRF_A::NotReceived,
            true => DEVADDRF_A::Received,
        }
    }
    #[doc = "SMBus device address is not received"]
    #[inline(always)]
    pub fn is_not_received(&self) -> bool {
        *self == DEVADDRF_A::NotReceived
    }
    #[doc = "SMBus device address is received"]
    #[inline(always)]
    pub fn is_received(&self) -> bool {
        *self == DEVADDRF_A::Received
    }
}
#[doc = "SMBus host address receiving\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HOSTADDRF_A {
    #[doc = "0: SMBus host address is not received"]
    NotReceived = 0,
    #[doc = "1: SMBus host address is received"]
    Received = 1,
}
impl From<HOSTADDRF_A> for bool {
    #[inline(always)]
    fn from(variant: HOSTADDRF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HOSTADDRF` reader - SMBus host address receiving"]
pub type HOSTADDRF_R = crate::BitReader<HOSTADDRF_A>;
impl HOSTADDRF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HOSTADDRF_A {
        match self.bits {
            false => HOSTADDRF_A::NotReceived,
            true => HOSTADDRF_A::Received,
        }
    }
    #[doc = "SMBus host address is not received"]
    #[inline(always)]
    pub fn is_not_received(&self) -> bool {
        *self == HOSTADDRF_A::NotReceived
    }
    #[doc = "SMBus host address is received"]
    #[inline(always)]
    pub fn is_received(&self) -> bool {
        *self == HOSTADDRF_A::Received
    }
}
#[doc = "Received address 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADDR2F_A {
    #[doc = "0: Received address matches the contents of OADDR1"]
    Oaddr1 = 0,
    #[doc = "1: Received address matches the contents of OADDR2"]
    Oaddr2 = 1,
}
impl From<ADDR2F_A> for bool {
    #[inline(always)]
    fn from(variant: ADDR2F_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADDR2F` reader - Received address 2"]
pub type ADDR2F_R = crate::BitReader<ADDR2F_A>;
impl ADDR2F_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ADDR2F_A {
        match self.bits {
            false => ADDR2F_A::Oaddr1,
            true => ADDR2F_A::Oaddr2,
        }
    }
    #[doc = "Received address matches the contents of OADDR1"]
    #[inline(always)]
    pub fn is_oaddr1(&self) -> bool {
        *self == ADDR2F_A::Oaddr1
    }
    #[doc = "Received address matches the contents of OADDR2"]
    #[inline(always)]
    pub fn is_oaddr2(&self) -> bool {
        *self == ADDR2F_A::Oaddr2
    }
}
#[doc = "Field `PECVAL` reader - PEC value"]
pub type PECVAL_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Transmission mode"]
    #[inline(always)]
    pub fn trmode(&self) -> TRMODE_R {
        TRMODE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Bus busy"]
    #[inline(always)]
    pub fn busyf(&self) -> BUSYF_R {
        BUSYF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmission direction"]
    #[inline(always)]
    pub fn dirf(&self) -> DIRF_R {
        DIRF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - General call address reception"]
    #[inline(always)]
    pub fn gcaddrf(&self) -> GCADDRF_R {
        GCADDRF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SMBus device address receiving"]
    #[inline(always)]
    pub fn devaddrf(&self) -> DEVADDRF_R {
        DEVADDRF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SMBus host address receiving"]
    #[inline(always)]
    pub fn hostaddrf(&self) -> HOSTADDRF_R {
        HOSTADDRF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Received address 2"]
    #[inline(always)]
    pub fn addr2f(&self) -> ADDR2F_R {
        ADDR2F_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:15 - PEC value"]
    #[inline(always)]
    pub fn pecval(&self) -> PECVAL_R {
        PECVAL_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STS2")
            .field("pecval", &self.pecval())
            .field("addr2f", &self.addr2f())
            .field("hostaddrf", &self.hostaddrf())
            .field("devaddrf", &self.devaddrf())
            .field("gcaddrf", &self.gcaddrf())
            .field("dirf", &self.dirf())
            .field("busyf", &self.busyf())
            .field("trmode", &self.trmode())
            .finish()
    }
}
#[doc = "Status register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`sts2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STS2_SPEC;
impl crate::RegisterSpec for STS2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sts2::R`](R) reader structure"]
impl crate::Readable for STS2_SPEC {}
#[doc = "`reset()` method sets STS2 to value 0"]
impl crate::Resettable for STS2_SPEC {}
