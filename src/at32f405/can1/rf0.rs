#[doc = "Register `RF0` reader"]
pub type R = crate::R<RF0_SPEC>;
#[doc = "Register `RF0` writer"]
pub type W = crate::W<RF0_SPEC>;
#[doc = "Field `RF0MN` reader - Receive FIFO 0 message num"]
pub type RF0MN_R = crate::FieldReader;
#[doc = "Field `RF0FF` reader - Receive FIFO 0 full flag"]
pub type RF0FF_R = crate::BitReader;
#[doc = "Field `RF0FF` writer - Receive FIFO 0 full flag"]
pub type RF0FF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RF0OF` reader - Receive FIFO 0 overflow flag"]
pub type RF0OF_R = crate::BitReader;
#[doc = "Field `RF0OF` writer - Receive FIFO 0 overflow flag"]
pub type RF0OF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RF0R` reader - Receive FIFO 0 release"]
pub type RF0R_R = crate::BitReader;
#[doc = "Field `RF0R` writer - Receive FIFO 0 release"]
pub type RF0R_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:1 - Receive FIFO 0 message num"]
    #[inline(always)]
    pub fn rf0mn(&self) -> RF0MN_R {
        RF0MN_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 3 - Receive FIFO 0 full flag"]
    #[inline(always)]
    pub fn rf0ff(&self) -> RF0FF_R {
        RF0FF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive FIFO 0 overflow flag"]
    #[inline(always)]
    pub fn rf0of(&self) -> RF0OF_R {
        RF0OF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Receive FIFO 0 release"]
    #[inline(always)]
    pub fn rf0r(&self) -> RF0R_R {
        RF0R_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - Receive FIFO 0 full flag"]
    #[inline(always)]
    #[must_use]
    pub fn rf0ff(&mut self) -> RF0FF_W<RF0_SPEC, 3> {
        RF0FF_W::new(self)
    }
    #[doc = "Bit 4 - Receive FIFO 0 overflow flag"]
    #[inline(always)]
    #[must_use]
    pub fn rf0of(&mut self) -> RF0OF_W<RF0_SPEC, 4> {
        RF0OF_W::new(self)
    }
    #[doc = "Bit 5 - Receive FIFO 0 release"]
    #[inline(always)]
    #[must_use]
    pub fn rf0r(&mut self) -> RF0R_W<RF0_SPEC, 5> {
        RF0R_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Receive FIFO 0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rf0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rf0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RF0_SPEC;
impl crate::RegisterSpec for RF0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rf0::R`](R) reader structure"]
impl crate::Readable for RF0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rf0::W`](W) writer structure"]
impl crate::Writable for RF0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RF0 to value 0"]
impl crate::Resettable for RF0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
