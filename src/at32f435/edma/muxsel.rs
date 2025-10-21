#[doc = "Register `MUXSEL` reader"]
pub type R = crate::R<MUXSEL_SPEC>;
#[doc = "Register `MUXSEL` writer"]
pub type W = crate::W<MUXSEL_SPEC>;
#[doc = "Field `TBL_SEL` reader - Multiplexer Table Select"]
pub type TBL_SEL_R = crate::BitReader;
#[doc = "Field `TBL_SEL` writer - Multiplexer Table Select"]
pub type TBL_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Multiplexer Table Select"]
    #[inline(always)]
    pub fn tbl_sel(&self) -> TBL_SEL_R {
        TBL_SEL_R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MUXSEL")
            .field("tbl_sel", &self.tbl_sel())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Multiplexer Table Select"]
    #[inline(always)]
    pub fn tbl_sel(&mut self) -> TBL_SEL_W<'_, MUXSEL_SPEC> {
        TBL_SEL_W::new(self, 0)
    }
}
#[doc = "EDMA MUX Table Selection\n\nYou can [`read`](crate::Reg::read) this register and get [`muxsel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`muxsel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MUXSEL_SPEC;
impl crate::RegisterSpec for MUXSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`muxsel::R`](R) reader structure"]
impl crate::Readable for MUXSEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`muxsel::W`](W) writer structure"]
impl crate::Writable for MUXSEL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MUXSEL to value 0"]
impl crate::Resettable for MUXSEL_SPEC {}
