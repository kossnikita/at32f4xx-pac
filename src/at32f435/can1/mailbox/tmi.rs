#[doc = "Register `TMI` reader"]
pub type R = crate::R<TMI_SPEC>;
#[doc = "Register `TMI` writer"]
pub type W = crate::W<TMI_SPEC>;
#[doc = "Transmit mailbox send request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Srr {
    #[doc = "0: Transmission in progress"]
    InProgress = 0,
    #[doc = "1: Transmission completed"]
    Completed = 1,
}
impl From<Srr> for bool {
    #[inline(always)]
    fn from(variant: Srr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SR` reader - Transmit mailbox send request"]
pub type SR_R = crate::BitReader<Srr>;
impl SR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Srr {
        match self.bits {
            false => Srr::InProgress,
            true => Srr::Completed,
        }
    }
    #[doc = "Transmission in progress"]
    #[inline(always)]
    pub fn is_in_progress(&self) -> bool {
        *self == Srr::InProgress
    }
    #[doc = "Transmission completed"]
    #[inline(always)]
    pub fn is_completed(&self) -> bool {
        *self == Srr::Completed
    }
}
#[doc = "Transmit mailbox send request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SrwWO {
    #[doc = "1: Transmit request"]
    Transmit = 1,
}
impl From<SrwWO> for bool {
    #[inline(always)]
    fn from(variant: SrwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SR` writer - Transmit mailbox send request"]
pub type SR_W<'a, REG> = crate::BitWriter1S<'a, REG, SrwWO>;
impl<'a, REG> SR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Transmit request"]
    #[inline(always)]
    pub fn transmit(self) -> &'a mut crate::W<REG> {
        self.variant(SrwWO::Transmit)
    }
}
#[doc = "Transmit mailbox frame type select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FRSEL_A {
    #[doc = "0: Data frame"]
    Data = 0,
    #[doc = "1: Remote frame"]
    Remote = 1,
}
impl From<FRSEL_A> for bool {
    #[inline(always)]
    fn from(variant: FRSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FRSEL` reader - Transmit mailbox frame type select"]
pub type FRSEL_R = crate::BitReader<FRSEL_A>;
impl FRSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FRSEL_A {
        match self.bits {
            false => FRSEL_A::Data,
            true => FRSEL_A::Remote,
        }
    }
    #[doc = "Data frame"]
    #[inline(always)]
    pub fn is_data(&self) -> bool {
        *self == FRSEL_A::Data
    }
    #[doc = "Remote frame"]
    #[inline(always)]
    pub fn is_remote(&self) -> bool {
        *self == FRSEL_A::Remote
    }
}
#[doc = "Field `FRSEL` writer - Transmit mailbox frame type select"]
pub type FRSEL_W<'a, REG> = crate::BitWriter<'a, REG, FRSEL_A>;
impl<'a, REG> FRSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Data frame"]
    #[inline(always)]
    pub fn data(self) -> &'a mut crate::W<REG> {
        self.variant(FRSEL_A::Data)
    }
    #[doc = "Remote frame"]
    #[inline(always)]
    pub fn remote(self) -> &'a mut crate::W<REG> {
        self.variant(FRSEL_A::Remote)
    }
}
#[doc = "Transmit mailbox identifier type select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IDSEL_A {
    #[doc = "0: Standard identifier"]
    Standard = 0,
    #[doc = "1: Extended identifier"]
    Extended = 1,
}
impl From<IDSEL_A> for bool {
    #[inline(always)]
    fn from(variant: IDSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDSEL` reader - Transmit mailbox identifier type select"]
pub type IDSEL_R = crate::BitReader<IDSEL_A>;
impl IDSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IDSEL_A {
        match self.bits {
            false => IDSEL_A::Standard,
            true => IDSEL_A::Extended,
        }
    }
    #[doc = "Standard identifier"]
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == IDSEL_A::Standard
    }
    #[doc = "Extended identifier"]
    #[inline(always)]
    pub fn is_extended(&self) -> bool {
        *self == IDSEL_A::Extended
    }
}
#[doc = "Field `IDSEL` writer - Transmit mailbox identifier type select"]
pub type IDSEL_W<'a, REG> = crate::BitWriter<'a, REG, IDSEL_A>;
impl<'a, REG> IDSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Standard identifier"]
    #[inline(always)]
    pub fn standard(self) -> &'a mut crate::W<REG> {
        self.variant(IDSEL_A::Standard)
    }
    #[doc = "Extended identifier"]
    #[inline(always)]
    pub fn extended(self) -> &'a mut crate::W<REG> {
        self.variant(IDSEL_A::Extended)
    }
}
#[doc = "Field `EID` reader - Ttransmit mailbox extended identifier"]
pub type EID_R = crate::FieldReader<u32>;
#[doc = "Field `EID` writer - Ttransmit mailbox extended identifier"]
pub type EID_W<'a, REG> = crate::FieldWriter<'a, REG, 18, u32, crate::Safe>;
#[doc = "Field `SID` reader - Transmit mailbox standard identifier or extended identifier high bytes"]
pub type SID_R = crate::FieldReader<u16>;
#[doc = "Field `SID` writer - Transmit mailbox standard identifier or extended identifier high bytes"]
pub type SID_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16, crate::Safe>;
impl R {
    #[doc = "Bit 0 - Transmit mailbox send request"]
    #[inline(always)]
    pub fn sr(&self) -> SR_R {
        SR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit mailbox frame type select"]
    #[inline(always)]
    pub fn frsel(&self) -> FRSEL_R {
        FRSEL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmit mailbox identifier type select"]
    #[inline(always)]
    pub fn idsel(&self) -> IDSEL_R {
        IDSEL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:20 - Ttransmit mailbox extended identifier"]
    #[inline(always)]
    pub fn eid(&self) -> EID_R {
        EID_R::new((self.bits >> 3) & 0x0003_ffff)
    }
    #[doc = "Bits 21:31 - Transmit mailbox standard identifier or extended identifier high bytes"]
    #[inline(always)]
    pub fn sid(&self) -> SID_R {
        SID_R::new(((self.bits >> 21) & 0x07ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TMI")
            .field("sid", &self.sid())
            .field("eid", &self.eid())
            .field("idsel", &self.idsel())
            .field("frsel", &self.frsel())
            .field("sr", &self.sr())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Transmit mailbox send request"]
    #[inline(always)]
    pub fn sr(&mut self) -> SR_W<'_, TMI_SPEC> {
        SR_W::new(self, 0)
    }
    #[doc = "Bit 1 - Transmit mailbox frame type select"]
    #[inline(always)]
    pub fn frsel(&mut self) -> FRSEL_W<'_, TMI_SPEC> {
        FRSEL_W::new(self, 1)
    }
    #[doc = "Bit 2 - Transmit mailbox identifier type select"]
    #[inline(always)]
    pub fn idsel(&mut self) -> IDSEL_W<'_, TMI_SPEC> {
        IDSEL_W::new(self, 2)
    }
    #[doc = "Bits 3:20 - Ttransmit mailbox extended identifier"]
    #[inline(always)]
    pub fn eid(&mut self) -> EID_W<'_, TMI_SPEC> {
        EID_W::new(self, 3)
    }
    #[doc = "Bits 21:31 - Transmit mailbox standard identifier or extended identifier high bytes"]
    #[inline(always)]
    pub fn sid(&mut self) -> SID_W<'_, TMI_SPEC> {
        SID_W::new(self, 21)
    }
}
#[doc = "Transmit mailbox identifier register\n\nYou can [`read`](crate::Reg::read) this register and get [`tmi::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmi::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TMI_SPEC;
impl crate::RegisterSpec for TMI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tmi::R`](R) reader structure"]
impl crate::Readable for TMI_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tmi::W`](W) writer structure"]
impl crate::Writable for TMI_SPEC {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x01;
}
#[doc = "`reset()` method sets TMI to value 0"]
impl crate::Resettable for TMI_SPEC {}
