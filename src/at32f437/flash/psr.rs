#[doc = "Register `PSR` reader"]
pub type R = crate::R<PSR_SPEC>;
#[doc = "Register `PSR` writer"]
pub type W = crate::W<PSR_SPEC>;
#[doc = "Field `NZW_BST` reader - Flash non-zero wait area boost"]
pub type NZW_BST_R = crate::BitReader;
#[doc = "Field `NZW_BST` writer - Flash non-zero wait area boost"]
pub type NZW_BST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `NZW_BST_STS` reader - Flash non-zero wait area boost status"]
pub type NZW_BST_STS_R = crate::BitReader;
impl R {
    #[doc = "Bit 12 - Flash non-zero wait area boost"]
    #[inline(always)]
    pub fn nzw_bst(&self) -> NZW_BST_R {
        NZW_BST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Flash non-zero wait area boost status"]
    #[inline(always)]
    pub fn nzw_bst_sts(&self) -> NZW_BST_STS_R {
        NZW_BST_STS_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PSR")
            .field("nzw_bst_sts", &format_args!("{}", self.nzw_bst_sts().bit()))
            .field("nzw_bst", &format_args!("{}", self.nzw_bst().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<PSR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 12 - Flash non-zero wait area boost"]
    #[inline(always)]
    #[must_use]
    pub fn nzw_bst(&mut self) -> NZW_BST_W<PSR_SPEC, 12> {
        NZW_BST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Performance selection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`psr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`psr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PSR_SPEC;
impl crate::RegisterSpec for PSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`psr::R`](R) reader structure"]
impl crate::Readable for PSR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`psr::W`](W) writer structure"]
impl crate::Writable for PSR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PSR to value 0x0330"]
impl crate::Resettable for PSR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0330;
}
