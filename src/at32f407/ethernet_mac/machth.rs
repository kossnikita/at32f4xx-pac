#[doc = "Register `MACHTH` reader"]
pub type R = crate::R<MACHTH_SPEC>;
#[doc = "Register `MACHTH` writer"]
pub type W = crate::W<MACHTH_SPEC>;
#[doc = "Field `HTH` reader - Hash table high"]
pub type HTH_R = crate::FieldReader<u32>;
#[doc = "Field `HTH` writer - Hash table high"]
pub type HTH_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Hash table high"]
    #[inline(always)]
    pub fn hth(&self) -> HTH_R {
        HTH_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Hash table high"]
    #[inline(always)]
    #[must_use]
    pub fn hth(&mut self) -> HTH_W<MACHTH_SPEC, 0> {
        HTH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Ethernet MAC hash table high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`machth::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`machth::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACHTH_SPEC;
impl crate::RegisterSpec for MACHTH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`machth::R`](R) reader structure"]
impl crate::Readable for MACHTH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`machth::W`](W) writer structure"]
impl crate::Writable for MACHTH_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MACHTH to value 0"]
impl crate::Resettable for MACHTH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
