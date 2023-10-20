#[doc = "Register `HCINT2` reader"]
pub type R = crate::R<HCINT2_SPEC>;
#[doc = "Register `HCINT2` writer"]
pub type W = crate::W<HCINT2_SPEC>;
#[doc = "Field `XFERC` reader - Transfer completed"]
pub type XFERC_R = crate::BitReader;
#[doc = "Field `XFERC` writer - Transfer completed"]
pub type XFERC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHHLTD` reader - Channel halted"]
pub type CHHLTD_R = crate::BitReader;
#[doc = "Field `CHHLTD` writer - Channel halted"]
pub type CHHLTD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STALL` reader - STALL response received interrupt"]
pub type STALL_R = crate::BitReader;
#[doc = "Field `STALL` writer - STALL response received interrupt"]
pub type STALL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NAK` reader - NAK response received interrupt"]
pub type NAK_R = crate::BitReader;
#[doc = "Field `NAK` writer - NAK response received interrupt"]
pub type NAK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACK` reader - ACK response received/transmitted interrupt"]
pub type ACK_R = crate::BitReader;
#[doc = "Field `ACK` writer - ACK response received/transmitted interrupt"]
pub type ACK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XACTERR` reader - Transaction error"]
pub type XACTERR_R = crate::BitReader;
#[doc = "Field `XACTERR` writer - Transaction error"]
pub type XACTERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BBLERR` reader - Babble error"]
pub type BBLERR_R = crate::BitReader;
#[doc = "Field `BBLERR` writer - Babble error"]
pub type BBLERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRMOVRUN` reader - Frame overrun"]
pub type FRMOVRUN_R = crate::BitReader;
#[doc = "Field `FRMOVRUN` writer - Frame overrun"]
pub type FRMOVRUN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTGLERR` reader - Data toggle error"]
pub type DTGLERR_R = crate::BitReader;
#[doc = "Field `DTGLERR` writer - Data toggle error"]
pub type DTGLERR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Transfer completed"]
    #[inline(always)]
    pub fn xferc(&self) -> XFERC_R {
        XFERC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel halted"]
    #[inline(always)]
    pub fn chhltd(&self) -> CHHLTD_R {
        CHHLTD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - STALL response received interrupt"]
    #[inline(always)]
    pub fn stall(&self) -> STALL_R {
        STALL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - NAK response received interrupt"]
    #[inline(always)]
    pub fn nak(&self) -> NAK_R {
        NAK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ACK response received/transmitted interrupt"]
    #[inline(always)]
    pub fn ack(&self) -> ACK_R {
        ACK_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Transaction error"]
    #[inline(always)]
    pub fn xacterr(&self) -> XACTERR_R {
        XACTERR_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Babble error"]
    #[inline(always)]
    pub fn bblerr(&self) -> BBLERR_R {
        BBLERR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Frame overrun"]
    #[inline(always)]
    pub fn frmovrun(&self) -> FRMOVRUN_R {
        FRMOVRUN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Data toggle error"]
    #[inline(always)]
    pub fn dtglerr(&self) -> DTGLERR_R {
        DTGLERR_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HCINT2")
            .field("xferc", &format_args!("{}", self.xferc().bit()))
            .field("chhltd", &format_args!("{}", self.chhltd().bit()))
            .field("stall", &format_args!("{}", self.stall().bit()))
            .field("nak", &format_args!("{}", self.nak().bit()))
            .field("ack", &format_args!("{}", self.ack().bit()))
            .field("xacterr", &format_args!("{}", self.xacterr().bit()))
            .field("bblerr", &format_args!("{}", self.bblerr().bit()))
            .field("frmovrun", &format_args!("{}", self.frmovrun().bit()))
            .field("dtglerr", &format_args!("{}", self.dtglerr().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<HCINT2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Transfer completed"]
    #[inline(always)]
    #[must_use]
    pub fn xferc(&mut self) -> XFERC_W<HCINT2_SPEC> {
        XFERC_W::new(self, 0)
    }
    #[doc = "Bit 1 - Channel halted"]
    #[inline(always)]
    #[must_use]
    pub fn chhltd(&mut self) -> CHHLTD_W<HCINT2_SPEC> {
        CHHLTD_W::new(self, 1)
    }
    #[doc = "Bit 3 - STALL response received interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn stall(&mut self) -> STALL_W<HCINT2_SPEC> {
        STALL_W::new(self, 3)
    }
    #[doc = "Bit 4 - NAK response received interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn nak(&mut self) -> NAK_W<HCINT2_SPEC> {
        NAK_W::new(self, 4)
    }
    #[doc = "Bit 5 - ACK response received/transmitted interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ack(&mut self) -> ACK_W<HCINT2_SPEC> {
        ACK_W::new(self, 5)
    }
    #[doc = "Bit 7 - Transaction error"]
    #[inline(always)]
    #[must_use]
    pub fn xacterr(&mut self) -> XACTERR_W<HCINT2_SPEC> {
        XACTERR_W::new(self, 7)
    }
    #[doc = "Bit 8 - Babble error"]
    #[inline(always)]
    #[must_use]
    pub fn bblerr(&mut self) -> BBLERR_W<HCINT2_SPEC> {
        BBLERR_W::new(self, 8)
    }
    #[doc = "Bit 9 - Frame overrun"]
    #[inline(always)]
    #[must_use]
    pub fn frmovrun(&mut self) -> FRMOVRUN_W<HCINT2_SPEC> {
        FRMOVRUN_W::new(self, 9)
    }
    #[doc = "Bit 10 - Data toggle error"]
    #[inline(always)]
    #[must_use]
    pub fn dtglerr(&mut self) -> DTGLERR_W<HCINT2_SPEC> {
        DTGLERR_W::new(self, 10)
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
#[doc = "OTGFS host channel-2 interrupt register (OTGFS_HCINT2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcint2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcint2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HCINT2_SPEC;
impl crate::RegisterSpec for HCINT2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcint2::R`](R) reader structure"]
impl crate::Readable for HCINT2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hcint2::W`](W) writer structure"]
impl crate::Writable for HCINT2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HCINT2 to value 0"]
impl crate::Resettable for HCINT2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
