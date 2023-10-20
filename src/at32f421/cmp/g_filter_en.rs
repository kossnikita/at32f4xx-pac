#[doc = "Register `G_FILTER_EN` reader"]
pub type R = crate::R<G_FILTER_EN_SPEC>;
#[doc = "Register `G_FILTER_EN` writer"]
pub type W = crate::W<G_FILTER_EN_SPEC>;
#[doc = "Field `GFE` reader - Glitch filter enable"]
pub type GFE_R = crate::BitReader;
#[doc = "Field `GFE` writer - Glitch filter enable"]
pub type GFE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Glitch filter enable"]
    #[inline(always)]
    pub fn gfe(&self) -> GFE_R {
        GFE_R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("G_FILTER_EN")
            .field("gfe", &format_args!("{}", self.gfe().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<G_FILTER_EN_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Glitch filter enable"]
    #[inline(always)]
    #[must_use]
    pub fn gfe(&mut self) -> GFE_W<G_FILTER_EN_SPEC> {
        GFE_W::new(self, 0)
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
#[doc = "G_FILTER_EN\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`g_filter_en::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`g_filter_en::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct G_FILTER_EN_SPEC;
impl crate::RegisterSpec for G_FILTER_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`g_filter_en::R`](R) reader structure"]
impl crate::Readable for G_FILTER_EN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`g_filter_en::W`](W) writer structure"]
impl crate::Writable for G_FILTER_EN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets G_FILTER_EN to value 0"]
impl crate::Resettable for G_FILTER_EN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
