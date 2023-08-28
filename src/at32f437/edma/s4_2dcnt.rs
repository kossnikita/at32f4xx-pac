#[doc = "Register `S4_2DCNT` reader"]
pub type R = crate::R<S4_2DCNT_SPEC>;
#[doc = "Register `S4_2DCNT` writer"]
pub type W = crate::W<S4_2DCNT_SPEC>;
#[doc = "Field `XCONUT` reader - X dimension transfer count"]
pub type XCONUT_R = crate::FieldReader<u16>;
#[doc = "Field `XCONUT` writer - X dimension transfer count"]
pub type XCONUT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
#[doc = "Field `YCONUT` reader - Y dimension transfer count"]
pub type YCONUT_R = crate::FieldReader<u16>;
#[doc = "Field `YCONUT` writer - Y dimension transfer count"]
pub type YCONUT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - X dimension transfer count"]
    #[inline(always)]
    pub fn xconut(&self) -> XCONUT_R {
        XCONUT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Y dimension transfer count"]
    #[inline(always)]
    pub fn yconut(&self) -> YCONUT_R {
        YCONUT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - X dimension transfer count"]
    #[inline(always)]
    #[must_use]
    pub fn xconut(&mut self) -> XCONUT_W<S4_2DCNT_SPEC, 0> {
        XCONUT_W::new(self)
    }
    #[doc = "Bits 16:31 - Y dimension transfer count"]
    #[inline(always)]
    #[must_use]
    pub fn yconut(&mut self) -> YCONUT_W<S4_2DCNT_SPEC, 16> {
        YCONUT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Stream 4 2D Transfer Count\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s4_2dcnt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s4_2dcnt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct S4_2DCNT_SPEC;
impl crate::RegisterSpec for S4_2DCNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`s4_2dcnt::R`](R) reader structure"]
impl crate::Readable for S4_2DCNT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`s4_2dcnt::W`](W) writer structure"]
impl crate::Writable for S4_2DCNT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets S4_2DCNT to value 0"]
impl crate::Resettable for S4_2DCNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
