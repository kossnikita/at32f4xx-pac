#[doc = "Register `S7_2DCNT` reader"]
pub type R = crate::R<S7_2DCNT_SPEC>;
#[doc = "Register `S7_2DCNT` writer"]
pub type W = crate::W<S7_2DCNT_SPEC>;
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
        f.debug_struct("S7_2DCNT")
            .field("xconut", &self.xconut())
            .field("yconut", &self.yconut())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - X dimension transfer count"]
    #[inline(always)]
    #[must_use]
    pub fn xconut(&mut self) -> XCONUT_W<S7_2DCNT_SPEC> {
        XCONUT_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - Y dimension transfer count"]
    #[inline(always)]
    #[must_use]
    pub fn yconut(&mut self) -> YCONUT_W<S7_2DCNT_SPEC> {
        YCONUT_W::new(self, 16)
    }
}
#[doc = "Stream 7 2D Transfer Count\n\nYou can [`read`](crate::Reg::read) this register and get [`s7_2dcnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s7_2dcnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct S7_2DCNT_SPEC;
impl crate::RegisterSpec for S7_2DCNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`s7_2dcnt::R`](R) reader structure"]
impl crate::Readable for S7_2DCNT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`s7_2dcnt::W`](W) writer structure"]
impl crate::Writable for S7_2DCNT_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets S7_2DCNT to value 0"]
impl crate::Resettable for S7_2DCNT_SPEC {
    const RESET_VALUE: u32 = 0;
}
