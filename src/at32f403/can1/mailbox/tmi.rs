#[doc = "Register `TMI` reader"]
pub type R = crate::R<TMI_SPEC>;
#[doc = "Register `TMI` writer"]
pub type W = crate::W<TMI_SPEC>;
#[doc = "Field `SR` reader - Transmit mailbox send request"]
pub type SR_R = crate::BitReader<SRR_A>;
#[doc = "Transmit mailbox send request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRR_A {
    #[doc = "0: Transmission in progress"]
    InProgress = 0,
    #[doc = "1: Transmission completed"]
    Completed = 1,
}
impl From<SRR_A> for bool {
    #[inline(always)]
    fn from(variant: SRR_A) -> Self {
        variant as u8 != 0
    }
}
impl SR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SRR_A {
        match self.bits {
            false => SRR_A::InProgress,
            true => SRR_A::Completed,
        }
    }
    #[doc = "Transmission in progress"]
    #[inline(always)]
    pub fn is_in_progress(&self) -> bool {
        *self == SRR_A::InProgress
    }
    #[doc = "Transmission completed"]
    #[inline(always)]
    pub fn is_completed(&self) -> bool {
        *self == SRR_A::Completed
    }
}
#[doc = "Transmit mailbox send request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRW_AW {
    #[doc = "1: Transmit request"]
    Transmit = 1,
}
impl From<SRW_AW> for bool {
    #[inline(always)]
    fn from(variant: SRW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SR` writer - Transmit mailbox send request"]
pub type SR_W<'a, REG, const O: u8> = crate::BitWriter1S<'a, REG, O, SRW_AW>;
impl<'a, REG, const O: u8> SR_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Transmit request"]
    #[inline(always)]
    pub fn transmit(self) -> &'a mut crate::W<REG> {
        self.variant(SRW_AW::Transmit)
    }
}
#[doc = "Field `FRSEL` reader - Transmit mailbox frame type select"]
pub type FRSEL_R = crate::BitReader<FRSEL_A>;
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
pub type FRSEL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, FRSEL_A>;
impl<'a, REG, const O: u8> FRSEL_W<'a, REG, O>
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
#[doc = "Field `IDSEL` reader - Transmit mailbox identifier type select"]
pub type IDSEL_R = crate::BitReader<IDSEL_A>;
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
pub type IDSEL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, IDSEL_A>;
impl<'a, REG, const O: u8> IDSEL_W<'a, REG, O>
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
pub type EID_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 18, O, u32>;
#[doc = "Field `SID` reader - Transmit mailbox standard identifier or extended identifier high bytes"]
pub type SID_R = crate::FieldReader<u16>;
#[doc = "Field `SID` writer - Transmit mailbox standard identifier or extended identifier high bytes"]
pub type SID_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 11, O, u16>;
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
            .field("sid", &format_args!("{}", self.sid().bits()))
            .field("eid", &format_args!("{}", self.eid().bits()))
            .field("idsel", &format_args!("{}", self.idsel().bit()))
            .field("frsel", &format_args!("{}", self.frsel().bit()))
            .field("sr", &format_args!("{}", self.sr().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<TMI_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit mailbox send request"]
    #[inline(always)]
    #[must_use]
    pub fn sr(&mut self) -> SR_W<TMI_SPEC, 0> {
        SR_W::new(self)
    }
    #[doc = "Bit 1 - Transmit mailbox frame type select"]
    #[inline(always)]
    #[must_use]
    pub fn frsel(&mut self) -> FRSEL_W<TMI_SPEC, 1> {
        FRSEL_W::new(self)
    }
    #[doc = "Bit 2 - Transmit mailbox identifier type select"]
    #[inline(always)]
    #[must_use]
    pub fn idsel(&mut self) -> IDSEL_W<TMI_SPEC, 2> {
        IDSEL_W::new(self)
    }
    #[doc = "Bits 3:20 - Ttransmit mailbox extended identifier"]
    #[inline(always)]
    #[must_use]
    pub fn eid(&mut self) -> EID_W<TMI_SPEC, 3> {
        EID_W::new(self)
    }
    #[doc = "Bits 21:31 - Transmit mailbox standard identifier or extended identifier high bytes"]
    #[inline(always)]
    #[must_use]
    pub fn sid(&mut self) -> SID_W<TMI_SPEC, 21> {
        SID_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Transmit mailbox identifier register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tmi::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tmi::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TMI_SPEC;
impl crate::RegisterSpec for TMI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tmi::R`](R) reader structure"]
impl crate::Readable for TMI_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tmi::W`](W) writer structure"]
impl crate::Writable for TMI_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x01;
}
#[doc = "`reset()` method sets TMI to value 0"]
impl crate::Resettable for TMI_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
