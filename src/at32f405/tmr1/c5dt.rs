#[doc = "Register `C5DT` reader"]
pub type R = crate::R<C5DT_SPEC>;
#[doc = "Register `C5DT` writer"]
pub type W = crate::W<C5DT_SPEC>;
#[doc = "Field `C5DT` reader - Channel 5 data register"]
pub type C5DT_R = crate::FieldReader<u16>;
#[doc = "Field `C5DT` writer - Channel 5 data register"]
pub type C5DT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Channel 5 data register"]
    #[inline(always)]
    pub fn c5dt(&self) -> C5DT_R {
        C5DT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Channel 5 data register"]
    #[inline(always)]
    #[must_use]
    pub fn c5dt(&mut self) -> C5DT_W<C5DT_SPEC, 0> {
        C5DT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Channel 5 data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c5dt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c5dt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C5DT_SPEC;
impl crate::RegisterSpec for C5DT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c5dt::R`](R) reader structure"]
impl crate::Readable for C5DT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`c5dt::W`](W) writer structure"]
impl crate::Writable for C5DT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets C5DT to value 0"]
impl crate::Resettable for C5DT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
