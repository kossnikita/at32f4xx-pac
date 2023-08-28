#[doc = "Register `RF1` reader"]
pub type R = crate::R<RF1_SPEC>;
#[doc = "Register `RF1` writer"]
pub type W = crate::W<RF1_SPEC>;
#[doc = "Field `RF1MN` reader - Receive FIFO 1 message num"]
pub type RF1MN_R = crate::FieldReader;
#[doc = "Field `RF1FF` reader - Receive FIFO 1 full flag"]
pub type RF1FF_R = crate::BitReader;
#[doc = "Field `RF1FF` writer - Receive FIFO 1 full flag"]
pub type RF1FF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RF1OF` reader - Receive FIFO 1 overflow flag"]
pub type RF1OF_R = crate::BitReader;
#[doc = "Field `RF1OF` writer - Receive FIFO 1 overflow flag"]
pub type RF1OF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RF1R` reader - Receive FIFO 1 release"]
pub type RF1R_R = crate::BitReader;
#[doc = "Field `RF1R` writer - Receive FIFO 1 release"]
pub type RF1R_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:1 - Receive FIFO 1 message num"]
    #[inline(always)]
    pub fn rf1mn(&self) -> RF1MN_R {
        RF1MN_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 3 - Receive FIFO 1 full flag"]
    #[inline(always)]
    pub fn rf1ff(&self) -> RF1FF_R {
        RF1FF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive FIFO 1 overflow flag"]
    #[inline(always)]
    pub fn rf1of(&self) -> RF1OF_R {
        RF1OF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Receive FIFO 1 release"]
    #[inline(always)]
    pub fn rf1r(&self) -> RF1R_R {
        RF1R_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - Receive FIFO 1 full flag"]
    #[inline(always)]
    #[must_use]
    pub fn rf1ff(&mut self) -> RF1FF_W<RF1_SPEC, 3> {
        RF1FF_W::new(self)
    }
    #[doc = "Bit 4 - Receive FIFO 1 overflow flag"]
    #[inline(always)]
    #[must_use]
    pub fn rf1of(&mut self) -> RF1OF_W<RF1_SPEC, 4> {
        RF1OF_W::new(self)
    }
    #[doc = "Bit 5 - Receive FIFO 1 release"]
    #[inline(always)]
    #[must_use]
    pub fn rf1r(&mut self) -> RF1R_W<RF1_SPEC, 5> {
        RF1R_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Receive FIFO 1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rf1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rf1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RF1_SPEC;
impl crate::RegisterSpec for RF1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rf1::R`](R) reader structure"]
impl crate::Readable for RF1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rf1::W`](W) writer structure"]
impl crate::Writable for RF1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RF1 to value 0"]
impl crate::Resettable for RF1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
