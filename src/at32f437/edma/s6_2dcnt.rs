#[doc = "Register `S6_2DCNT` reader"]
pub type R = crate::R<S6_2DCNT_SPEC>;
#[doc = "Register `S6_2DCNT` writer"]
pub type W = crate::W<S6_2DCNT_SPEC>;
#[doc = "Field `XCONUT` reader - X dimension transfer count"]
pub type XCONUT_R = crate::FieldReader<u16>;
#[doc = "Field `XCONUT` writer - X dimension transfer count"]
pub type XCONUT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `YCONUT` reader - Y dimension transfer count"]
pub type YCONUT_R = crate::FieldReader<u16>;
#[doc = "Field `YCONUT` writer - Y dimension transfer count"]
pub type YCONUT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
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
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("S6_2DCNT")
            .field("xconut", &format_args!("{}", self.xconut().bits()))
            .field("yconut", &format_args!("{}", self.yconut().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<S6_2DCNT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:15 - X dimension transfer count"]
    #[inline(always)]
    #[must_use]
    pub fn xconut(&mut self) -> XCONUT_W<S6_2DCNT_SPEC> {
        XCONUT_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - Y dimension transfer count"]
    #[inline(always)]
    #[must_use]
    pub fn yconut(&mut self) -> YCONUT_W<S6_2DCNT_SPEC> {
        YCONUT_W::new(self, 16)
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
#[doc = "Stream 6 2D Transfer Count\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s6_2dcnt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s6_2dcnt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct S6_2DCNT_SPEC;
impl crate::RegisterSpec for S6_2DCNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`s6_2dcnt::R`](R) reader structure"]
impl crate::Readable for S6_2DCNT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`s6_2dcnt::W`](W) writer structure"]
impl crate::Writable for S6_2DCNT_SPEC {
    const ZEROS_BITMAP: Self::Ux = 0;
    const ONES_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets S6_2DCNT to value 0"]
impl crate::Resettable for S6_2DCNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
