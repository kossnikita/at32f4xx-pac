#[doc = "Register `EXT3` reader"]
pub type R = crate::R<EXT3_SPEC>;
#[doc = "Register `EXT3` writer"]
pub type W = crate::W<EXT3_SPEC>;
#[doc = "Field `BUSLATW2W` reader - Bus turnaround phase for consecutive write duration"]
pub type BUSLATW2W_R = crate::FieldReader;
#[doc = "Field `BUSLATW2W` writer - Bus turnaround phase for consecutive write duration"]
pub type BUSLATW2W_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BUSLATR2R` reader - Bus turnaround phase for consecutive read duration"]
pub type BUSLATR2R_R = crate::FieldReader;
#[doc = "Field `BUSLATR2R` writer - Bus turnaround phase for consecutive read duration"]
pub type BUSLATR2R_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Bus turnaround phase for consecutive write duration"]
    #[inline(always)]
    pub fn buslatw2w(&self) -> BUSLATW2W_R {
        BUSLATW2W_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Bus turnaround phase for consecutive read duration"]
    #[inline(always)]
    pub fn buslatr2r(&self) -> BUSLATR2R_R {
        BUSLATR2R_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXT3")
            .field("buslatw2w", &format_args!("{}", self.buslatw2w().bits()))
            .field("buslatr2r", &format_args!("{}", self.buslatr2r().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<EXT3_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - Bus turnaround phase for consecutive write duration"]
    #[inline(always)]
    #[must_use]
    pub fn buslatw2w(&mut self) -> BUSLATW2W_W<EXT3_SPEC> {
        BUSLATW2W_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Bus turnaround phase for consecutive read duration"]
    #[inline(always)]
    #[must_use]
    pub fn buslatr2r(&mut self) -> BUSLATR2R_W<EXT3_SPEC> {
        BUSLATR2R_W::new(self, 8)
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
#[doc = "externl timeing register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ext3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ext3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXT3_SPEC;
impl crate::RegisterSpec for EXT3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ext3::R`](R) reader structure"]
impl crate::Readable for EXT3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ext3::W`](W) writer structure"]
impl crate::Writable for EXT3_SPEC {
    const ZEROS_BITMAP: Self::Ux = 0;
    const ONES_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EXT3 to value 0x0808"]
impl crate::Resettable for EXT3_SPEC {
    const RESET_VALUE: Self::Ux = 0x0808;
}
