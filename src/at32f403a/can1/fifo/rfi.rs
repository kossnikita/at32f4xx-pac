#[doc = "Register `RFI` reader"]
pub type R = crate::R<RFI_SPEC>;
#[doc = "Receive FIFO frame type indication\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FRI_A {
    #[doc = "0: Data frame"]
    Data = 0,
    #[doc = "1: Remote frame"]
    Remote = 1,
}
impl From<FRI_A> for bool {
    #[inline(always)]
    fn from(variant: FRI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FRI` reader - Receive FIFO frame type indication"]
pub type FRI_R = crate::BitReader<FRI_A>;
impl FRI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FRI_A {
        match self.bits {
            false => FRI_A::Data,
            true => FRI_A::Remote,
        }
    }
    #[doc = "Data frame"]
    #[inline(always)]
    pub fn is_data(&self) -> bool {
        *self == FRI_A::Data
    }
    #[doc = "Remote frame"]
    #[inline(always)]
    pub fn is_remote(&self) -> bool {
        *self == FRI_A::Remote
    }
}
#[doc = "Receive FIFO identifier type indication\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IDI_A {
    #[doc = "0: Standard identifier"]
    Standard = 0,
    #[doc = "1: Extended identifier"]
    Extended = 1,
}
impl From<IDI_A> for bool {
    #[inline(always)]
    fn from(variant: IDI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDI` reader - Receive FIFO identifier type indication"]
pub type IDI_R = crate::BitReader<IDI_A>;
impl IDI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IDI_A {
        match self.bits {
            false => IDI_A::Standard,
            true => IDI_A::Extended,
        }
    }
    #[doc = "Standard identifier"]
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == IDI_A::Standard
    }
    #[doc = "Extended identifier"]
    #[inline(always)]
    pub fn is_extended(&self) -> bool {
        *self == IDI_A::Extended
    }
}
#[doc = "Field `EID` reader - Receive FIFO extended identifier"]
pub type EID_R = crate::FieldReader<u32>;
#[doc = "Field `SID` reader - Receive FIFO standard identifier or receive FIFO extended identifier"]
pub type SID_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bit 1 - Receive FIFO frame type indication"]
    #[inline(always)]
    pub fn fri(&self) -> FRI_R {
        FRI_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receive FIFO identifier type indication"]
    #[inline(always)]
    pub fn idi(&self) -> IDI_R {
        IDI_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:20 - Receive FIFO extended identifier"]
    #[inline(always)]
    pub fn eid(&self) -> EID_R {
        EID_R::new((self.bits >> 3) & 0x0003_ffff)
    }
    #[doc = "Bits 21:31 - Receive FIFO standard identifier or receive FIFO extended identifier"]
    #[inline(always)]
    pub fn sid(&self) -> SID_R {
        SID_R::new(((self.bits >> 21) & 0x07ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RFI")
            .field("sid", &self.sid())
            .field("eid", &self.eid())
            .field("idi", &self.idi())
            .field("fri", &self.fri())
            .finish()
    }
}
#[doc = "Receive FIFO mailbox identifier register\n\nYou can [`read`](crate::Reg::read) this register and get [`rfi::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RFI_SPEC;
impl crate::RegisterSpec for RFI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rfi::R`](R) reader structure"]
impl crate::Readable for RFI_SPEC {}
#[doc = "`reset()` method sets RFI to value 0"]
impl crate::Resettable for RFI_SPEC {}
