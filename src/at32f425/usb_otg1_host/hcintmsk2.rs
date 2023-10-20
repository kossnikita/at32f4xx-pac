#[doc = "Register `HCINTMSK2` reader"]
pub type R = crate::R<HCINTMSK2_SPEC>;
#[doc = "Register `HCINTMSK2` writer"]
pub type W = crate::W<HCINTMSK2_SPEC>;
#[doc = "Field `XFERCMSK` reader - Transfer completed mask"]
pub type XFERCMSK_R = crate::BitReader;
#[doc = "Field `XFERCMSK` writer - Transfer completed mask"]
pub type XFERCMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHHLTDMSK` reader - Channel halted mask"]
pub type CHHLTDMSK_R = crate::BitReader;
#[doc = "Field `CHHLTDMSK` writer - Channel halted mask"]
pub type CHHLTDMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STALLMSK` reader - STALL response received interrupt mask"]
pub type STALLMSK_R = crate::BitReader;
#[doc = "Field `STALLMSK` writer - STALL response received interrupt mask"]
pub type STALLMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NAKMSK` reader - NAK response received interrupt mask"]
pub type NAKMSK_R = crate::BitReader;
#[doc = "Field `NAKMSK` writer - NAK response received interrupt mask"]
pub type NAKMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACKMSK` reader - ACK response received/transmitted interrupt mask"]
pub type ACKMSK_R = crate::BitReader;
#[doc = "Field `ACKMSK` writer - ACK response received/transmitted interrupt mask"]
pub type ACKMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XACTERRMSK` reader - Transaction error mask"]
pub type XACTERRMSK_R = crate::BitReader;
#[doc = "Field `XACTERRMSK` writer - Transaction error mask"]
pub type XACTERRMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BBLERRMSK` reader - Babble error mask"]
pub type BBLERRMSK_R = crate::BitReader;
#[doc = "Field `BBLERRMSK` writer - Babble error mask"]
pub type BBLERRMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRMOVRUNMSK` reader - Frame overrun mask"]
pub type FRMOVRUNMSK_R = crate::BitReader;
#[doc = "Field `FRMOVRUNMSK` writer - Frame overrun mask"]
pub type FRMOVRUNMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTGLERRMSK` reader - Data toggle error mask"]
pub type DTGLERRMSK_R = crate::BitReader;
#[doc = "Field `DTGLERRMSK` writer - Data toggle error mask"]
pub type DTGLERRMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Transfer completed mask"]
    #[inline(always)]
    pub fn xfercmsk(&self) -> XFERCMSK_R {
        XFERCMSK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel halted mask"]
    #[inline(always)]
    pub fn chhltdmsk(&self) -> CHHLTDMSK_R {
        CHHLTDMSK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - STALL response received interrupt mask"]
    #[inline(always)]
    pub fn stallmsk(&self) -> STALLMSK_R {
        STALLMSK_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - NAK response received interrupt mask"]
    #[inline(always)]
    pub fn nakmsk(&self) -> NAKMSK_R {
        NAKMSK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ACK response received/transmitted interrupt mask"]
    #[inline(always)]
    pub fn ackmsk(&self) -> ACKMSK_R {
        ACKMSK_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Transaction error mask"]
    #[inline(always)]
    pub fn xacterrmsk(&self) -> XACTERRMSK_R {
        XACTERRMSK_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Babble error mask"]
    #[inline(always)]
    pub fn bblerrmsk(&self) -> BBLERRMSK_R {
        BBLERRMSK_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Frame overrun mask"]
    #[inline(always)]
    pub fn frmovrunmsk(&self) -> FRMOVRUNMSK_R {
        FRMOVRUNMSK_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Data toggle error mask"]
    #[inline(always)]
    pub fn dtglerrmsk(&self) -> DTGLERRMSK_R {
        DTGLERRMSK_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HCINTMSK2")
            .field("xfercmsk", &format_args!("{}", self.xfercmsk().bit()))
            .field("chhltdmsk", &format_args!("{}", self.chhltdmsk().bit()))
            .field("stallmsk", &format_args!("{}", self.stallmsk().bit()))
            .field("nakmsk", &format_args!("{}", self.nakmsk().bit()))
            .field("ackmsk", &format_args!("{}", self.ackmsk().bit()))
            .field("xacterrmsk", &format_args!("{}", self.xacterrmsk().bit()))
            .field("bblerrmsk", &format_args!("{}", self.bblerrmsk().bit()))
            .field("frmovrunmsk", &format_args!("{}", self.frmovrunmsk().bit()))
            .field("dtglerrmsk", &format_args!("{}", self.dtglerrmsk().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<HCINTMSK2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Transfer completed mask"]
    #[inline(always)]
    #[must_use]
    pub fn xfercmsk(&mut self) -> XFERCMSK_W<HCINTMSK2_SPEC> {
        XFERCMSK_W::new(self, 0)
    }
    #[doc = "Bit 1 - Channel halted mask"]
    #[inline(always)]
    #[must_use]
    pub fn chhltdmsk(&mut self) -> CHHLTDMSK_W<HCINTMSK2_SPEC> {
        CHHLTDMSK_W::new(self, 1)
    }
    #[doc = "Bit 3 - STALL response received interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn stallmsk(&mut self) -> STALLMSK_W<HCINTMSK2_SPEC> {
        STALLMSK_W::new(self, 3)
    }
    #[doc = "Bit 4 - NAK response received interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn nakmsk(&mut self) -> NAKMSK_W<HCINTMSK2_SPEC> {
        NAKMSK_W::new(self, 4)
    }
    #[doc = "Bit 5 - ACK response received/transmitted interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn ackmsk(&mut self) -> ACKMSK_W<HCINTMSK2_SPEC> {
        ACKMSK_W::new(self, 5)
    }
    #[doc = "Bit 7 - Transaction error mask"]
    #[inline(always)]
    #[must_use]
    pub fn xacterrmsk(&mut self) -> XACTERRMSK_W<HCINTMSK2_SPEC> {
        XACTERRMSK_W::new(self, 7)
    }
    #[doc = "Bit 8 - Babble error mask"]
    #[inline(always)]
    #[must_use]
    pub fn bblerrmsk(&mut self) -> BBLERRMSK_W<HCINTMSK2_SPEC> {
        BBLERRMSK_W::new(self, 8)
    }
    #[doc = "Bit 9 - Frame overrun mask"]
    #[inline(always)]
    #[must_use]
    pub fn frmovrunmsk(&mut self) -> FRMOVRUNMSK_W<HCINTMSK2_SPEC> {
        FRMOVRUNMSK_W::new(self, 9)
    }
    #[doc = "Bit 10 - Data toggle error mask"]
    #[inline(always)]
    #[must_use]
    pub fn dtglerrmsk(&mut self) -> DTGLERRMSK_W<HCINTMSK2_SPEC> {
        DTGLERRMSK_W::new(self, 10)
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
#[doc = "OTGFS host channel-2 mask register (OTGFS_HCINTMSK2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcintmsk2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcintmsk2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HCINTMSK2_SPEC;
impl crate::RegisterSpec for HCINTMSK2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcintmsk2::R`](R) reader structure"]
impl crate::Readable for HCINTMSK2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hcintmsk2::W`](W) writer structure"]
impl crate::Writable for HCINTMSK2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HCINTMSK2 to value 0"]
impl crate::Resettable for HCINTMSK2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
