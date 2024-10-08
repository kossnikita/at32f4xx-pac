#[doc = "Register `HCFG` reader"]
pub type R = crate::R<HCFG_SPEC>;
#[doc = "Register `HCFG` writer"]
pub type W = crate::W<HCFG_SPEC>;
#[doc = "Field `FSLSPCLKSEL` reader - FS/LS PHY clock select"]
pub type FSLSPCLKSEL_R = crate::FieldReader;
#[doc = "Field `FSLSPCLKSEL` writer - FS/LS PHY clock select"]
pub type FSLSPCLKSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FSLSSUPP` reader - FS- and LS-only support"]
pub type FSLSSUPP_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:1 - FS/LS PHY clock select"]
    #[inline(always)]
    pub fn fslspclksel(&self) -> FSLSPCLKSEL_R {
        FSLSPCLKSEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - FS- and LS-only support"]
    #[inline(always)]
    pub fn fslssupp(&self) -> FSLSSUPP_R {
        FSLSSUPP_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HCFG")
            .field("fslspclksel", &self.fslspclksel())
            .field("fslssupp", &self.fslssupp())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - FS/LS PHY clock select"]
    #[inline(always)]
    #[must_use]
    pub fn fslspclksel(&mut self) -> FSLSPCLKSEL_W<HCFG_SPEC> {
        FSLSPCLKSEL_W::new(self, 0)
    }
}
#[doc = "OTGHS host configuration register (OTGHS_HCFG)\n\nYou can [`read`](crate::Reg::read) this register and get [`hcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HCFG_SPEC;
impl crate::RegisterSpec for HCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcfg::R`](R) reader structure"]
impl crate::Readable for HCFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hcfg::W`](W) writer structure"]
impl crate::Writable for HCFG_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HCFG to value 0"]
impl crate::Resettable for HCFG_SPEC {
    const RESET_VALUE: u32 = 0;
}
