#[doc = "Register `S1_2DCNT` reader"]
pub type R = crate::R<S1_2DCNT_SPEC>;
#[doc = "Register `S1_2DCNT` writer"]
pub type W = crate::W<S1_2DCNT_SPEC>;
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
        f.debug_struct("S1_2DCNT")
            .field("xconut", &self.xconut())
            .field("yconut", &self.yconut())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - X dimension transfer count"]
    #[inline(always)]
    pub fn xconut(&mut self) -> XCONUT_W<'_, S1_2DCNT_SPEC> {
        XCONUT_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - Y dimension transfer count"]
    #[inline(always)]
    pub fn yconut(&mut self) -> YCONUT_W<'_, S1_2DCNT_SPEC> {
        YCONUT_W::new(self, 16)
    }
}
#[doc = "Stream 1 2D Transfer Count\n\nYou can [`read`](crate::Reg::read) this register and get [`s1_2dcnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s1_2dcnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct S1_2DCNT_SPEC;
impl crate::RegisterSpec for S1_2DCNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`s1_2dcnt::R`](R) reader structure"]
impl crate::Readable for S1_2DCNT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`s1_2dcnt::W`](W) writer structure"]
impl crate::Writable for S1_2DCNT_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets S1_2DCNT to value 0"]
impl crate::Resettable for S1_2DCNT_SPEC {}
