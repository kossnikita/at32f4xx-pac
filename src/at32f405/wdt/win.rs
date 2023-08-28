#[doc = "Register `WIN` reader"]
pub type R = crate::R<WIN_SPEC>;
#[doc = "Register `WIN` writer"]
pub type W = crate::W<WIN_SPEC>;
#[doc = "Field `WIN` reader - Window value"]
pub type WIN_R = crate::FieldReader<u16>;
#[doc = "Field `WIN` writer - Window value"]
pub type WIN_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 12, O, u16>;
impl R {
    #[doc = "Bits 0:11 - Window value"]
    #[inline(always)]
    pub fn win(&self) -> WIN_R {
        WIN_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Window value"]
    #[inline(always)]
    #[must_use]
    pub fn win(&mut self) -> WIN_W<WIN_SPEC, 0> {
        WIN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Window register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WIN_SPEC;
impl crate::RegisterSpec for WIN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`win::R`](R) reader structure"]
impl crate::Readable for WIN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`win::W`](W) writer structure"]
impl crate::Writable for WIN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WIN to value 0x0fff"]
impl crate::Resettable for WIN_SPEC {
    const RESET_VALUE: Self::Ux = 0x0fff;
}
