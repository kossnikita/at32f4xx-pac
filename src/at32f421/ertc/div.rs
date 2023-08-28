#[doc = "Register `DIV` reader"]
pub type R = crate::R<DIV_SPEC>;
#[doc = "Register `DIV` writer"]
pub type W = crate::W<DIV_SPEC>;
#[doc = "Field `DIVB` reader - Diveder B"]
pub type DIVB_R = crate::FieldReader<u16>;
#[doc = "Field `DIVB` writer - Diveder B"]
pub type DIVB_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 15, O, u16>;
#[doc = "Field `DIVA` reader - Diveder A"]
pub type DIVA_R = crate::FieldReader;
#[doc = "Field `DIVA` writer - Diveder A"]
pub type DIVA_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
impl R {
    #[doc = "Bits 0:14 - Diveder B"]
    #[inline(always)]
    pub fn divb(&self) -> DIVB_R {
        DIVB_R::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bits 16:22 - Diveder A"]
    #[inline(always)]
    pub fn diva(&self) -> DIVA_R {
        DIVA_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:14 - Diveder B"]
    #[inline(always)]
    #[must_use]
    pub fn divb(&mut self) -> DIVB_W<DIV_SPEC, 0> {
        DIVB_W::new(self)
    }
    #[doc = "Bits 16:22 - Diveder A"]
    #[inline(always)]
    #[must_use]
    pub fn diva(&mut self) -> DIVA_W<DIV_SPEC, 16> {
        DIVA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Diveder register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`div::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`div::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIV_SPEC;
impl crate::RegisterSpec for DIV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`div::R`](R) reader structure"]
impl crate::Readable for DIV_SPEC {}
#[doc = "`write(|w| ..)` method takes [`div::W`](W) writer structure"]
impl crate::Writable for DIV_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DIV to value 0x007f_00ff"]
impl crate::Resettable for DIV_SPEC {
    const RESET_VALUE: Self::Ux = 0x007f_00ff;
}
