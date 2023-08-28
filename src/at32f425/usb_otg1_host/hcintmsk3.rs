#[doc = "Register `HCINTMSK3` reader"]
pub type R = crate::R<HCINTMSK3_SPEC>;
#[doc = "Register `HCINTMSK3` writer"]
pub type W = crate::W<HCINTMSK3_SPEC>;
#[doc = "Field `XFERCMSK` reader - Transfer completed mask"]
pub type XFERCMSK_R = crate::BitReader;
#[doc = "Field `XFERCMSK` writer - Transfer completed mask"]
pub type XFERCMSK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CHHLTDMSK` reader - Channel halted mask"]
pub type CHHLTDMSK_R = crate::BitReader;
#[doc = "Field `CHHLTDMSK` writer - Channel halted mask"]
pub type CHHLTDMSK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `STALLMSK` reader - STALL response received interrupt mask"]
pub type STALLMSK_R = crate::BitReader;
#[doc = "Field `STALLMSK` writer - STALL response received interrupt mask"]
pub type STALLMSK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `NAKMSK` reader - NAK response received interrupt mask"]
pub type NAKMSK_R = crate::BitReader;
#[doc = "Field `NAKMSK` writer - NAK response received interrupt mask"]
pub type NAKMSK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ACKMSK` reader - ACK response received/transmitted interrupt mask"]
pub type ACKMSK_R = crate::BitReader;
#[doc = "Field `ACKMSK` writer - ACK response received/transmitted interrupt mask"]
pub type ACKMSK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `XACTERRMSK` reader - Transaction error mask"]
pub type XACTERRMSK_R = crate::BitReader;
#[doc = "Field `XACTERRMSK` writer - Transaction error mask"]
pub type XACTERRMSK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BBLERRMSK` reader - Babble error mask"]
pub type BBLERRMSK_R = crate::BitReader;
#[doc = "Field `BBLERRMSK` writer - Babble error mask"]
pub type BBLERRMSK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FRMOVRUNMSK` reader - Frame overrun mask"]
pub type FRMOVRUNMSK_R = crate::BitReader;
#[doc = "Field `FRMOVRUNMSK` writer - Frame overrun mask"]
pub type FRMOVRUNMSK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DTGLERRMSK` reader - Data toggle error mask"]
pub type DTGLERRMSK_R = crate::BitReader;
#[doc = "Field `DTGLERRMSK` writer - Data toggle error mask"]
pub type DTGLERRMSK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
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
impl W {
    #[doc = "Bit 0 - Transfer completed mask"]
    #[inline(always)]
    #[must_use]
    pub fn xfercmsk(&mut self) -> XFERCMSK_W<HCINTMSK3_SPEC, 0> {
        XFERCMSK_W::new(self)
    }
    #[doc = "Bit 1 - Channel halted mask"]
    #[inline(always)]
    #[must_use]
    pub fn chhltdmsk(&mut self) -> CHHLTDMSK_W<HCINTMSK3_SPEC, 1> {
        CHHLTDMSK_W::new(self)
    }
    #[doc = "Bit 3 - STALL response received interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn stallmsk(&mut self) -> STALLMSK_W<HCINTMSK3_SPEC, 3> {
        STALLMSK_W::new(self)
    }
    #[doc = "Bit 4 - NAK response received interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn nakmsk(&mut self) -> NAKMSK_W<HCINTMSK3_SPEC, 4> {
        NAKMSK_W::new(self)
    }
    #[doc = "Bit 5 - ACK response received/transmitted interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn ackmsk(&mut self) -> ACKMSK_W<HCINTMSK3_SPEC, 5> {
        ACKMSK_W::new(self)
    }
    #[doc = "Bit 7 - Transaction error mask"]
    #[inline(always)]
    #[must_use]
    pub fn xacterrmsk(&mut self) -> XACTERRMSK_W<HCINTMSK3_SPEC, 7> {
        XACTERRMSK_W::new(self)
    }
    #[doc = "Bit 8 - Babble error mask"]
    #[inline(always)]
    #[must_use]
    pub fn bblerrmsk(&mut self) -> BBLERRMSK_W<HCINTMSK3_SPEC, 8> {
        BBLERRMSK_W::new(self)
    }
    #[doc = "Bit 9 - Frame overrun mask"]
    #[inline(always)]
    #[must_use]
    pub fn frmovrunmsk(&mut self) -> FRMOVRUNMSK_W<HCINTMSK3_SPEC, 9> {
        FRMOVRUNMSK_W::new(self)
    }
    #[doc = "Bit 10 - Data toggle error mask"]
    #[inline(always)]
    #[must_use]
    pub fn dtglerrmsk(&mut self) -> DTGLERRMSK_W<HCINTMSK3_SPEC, 10> {
        DTGLERRMSK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "OTGFS host channel-3 mask register (OTGFS_HCINTMSK3)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcintmsk3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcintmsk3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HCINTMSK3_SPEC;
impl crate::RegisterSpec for HCINTMSK3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcintmsk3::R`](R) reader structure"]
impl crate::Readable for HCINTMSK3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hcintmsk3::W`](W) writer structure"]
impl crate::Writable for HCINTMSK3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HCINTMSK3 to value 0"]
impl crate::Resettable for HCINTMSK3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
