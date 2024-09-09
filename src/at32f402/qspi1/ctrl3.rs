#[doc = "Register `CTRL3` reader"]
pub type R = crate::R<CTRL3_SPEC>;
#[doc = "Register `CTRL3` writer"]
pub type W = crate::W<CTRL3_SPEC>;
#[doc = "Field `ISPC` reader - Input sampling phase correction enable"]
pub type ISPC_R = crate::BitReader;
#[doc = "Field `ISPC` writer - Input sampling phase correction enable"]
pub type ISPC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 8 - Input sampling phase correction enable"]
    #[inline(always)]
    pub fn ispc(&self) -> ISPC_R {
        ISPC_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL3").field("ispc", &self.ispc()).finish()
    }
}
impl W {
    #[doc = "Bit 8 - Input sampling phase correction enable"]
    #[inline(always)]
    #[must_use]
    pub fn ispc(&mut self) -> ISPC_W<CTRL3_SPEC> {
        ISPC_W::new(self, 8)
    }
}
#[doc = "control register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL3_SPEC;
impl crate::RegisterSpec for CTRL3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl3::R`](R) reader structure"]
impl crate::Readable for CTRL3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl3::W`](W) writer structure"]
impl crate::Writable for CTRL3_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL3 to value 0"]
impl crate::Resettable for CTRL3_SPEC {
    const RESET_VALUE: u32 = 0;
}
