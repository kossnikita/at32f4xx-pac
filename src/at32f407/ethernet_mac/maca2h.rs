#[doc = "Register `MACA2H` reader"]
pub type R = crate::R<MACA2H_SPEC>;
#[doc = "Register `MACA2H` writer"]
pub type W = crate::W<MACA2H_SPEC>;
#[doc = "Field `MA2H` reader - MAC address 2 high"]
pub type MA2H_R = crate::FieldReader<u16>;
#[doc = "Field `MA2H` writer - MAC address 2 high"]
pub type MA2H_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
#[doc = "Field `MBC` reader - Mask byte control"]
pub type MBC_R = crate::FieldReader;
#[doc = "Field `MBC` writer - Mask byte control"]
pub type MBC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
#[doc = "Field `SA` reader - Source address"]
pub type SA_R = crate::BitReader;
#[doc = "Field `SA` writer - Source address"]
pub type SA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `AE` reader - Address enable"]
pub type AE_R = crate::BitReader;
#[doc = "Field `AE` writer - Address enable"]
pub type AE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:15 - MAC address 2 high"]
    #[inline(always)]
    pub fn ma2h(&self) -> MA2H_R {
        MA2H_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 24:29 - Mask byte control"]
    #[inline(always)]
    pub fn mbc(&self) -> MBC_R {
        MBC_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bit 30 - Source address"]
    #[inline(always)]
    pub fn sa(&self) -> SA_R {
        SA_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Address enable"]
    #[inline(always)]
    pub fn ae(&self) -> AE_R {
        AE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - MAC address 2 high"]
    #[inline(always)]
    #[must_use]
    pub fn ma2h(&mut self) -> MA2H_W<MACA2H_SPEC, 0> {
        MA2H_W::new(self)
    }
    #[doc = "Bits 24:29 - Mask byte control"]
    #[inline(always)]
    #[must_use]
    pub fn mbc(&mut self) -> MBC_W<MACA2H_SPEC, 24> {
        MBC_W::new(self)
    }
    #[doc = "Bit 30 - Source address"]
    #[inline(always)]
    #[must_use]
    pub fn sa(&mut self) -> SA_W<MACA2H_SPEC, 30> {
        SA_W::new(self)
    }
    #[doc = "Bit 31 - Address enable"]
    #[inline(always)]
    #[must_use]
    pub fn ae(&mut self) -> AE_W<MACA2H_SPEC, 31> {
        AE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Ethernet MAC address 2 high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maca2h::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maca2h::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACA2H_SPEC;
impl crate::RegisterSpec for MACA2H_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`maca2h::R`](R) reader structure"]
impl crate::Readable for MACA2H_SPEC {}
#[doc = "`write(|w| ..)` method takes [`maca2h::W`](W) writer structure"]
impl crate::Writable for MACA2H_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MACA2H to value 0x50"]
impl crate::Resettable for MACA2H_SPEC {
    const RESET_VALUE: Self::Ux = 0x50;
}