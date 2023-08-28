#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CTRL_SPEC>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CTRL_SPEC>;
#[doc = "Field `FPRGM` reader - Flash program"]
pub type FPRGM_R = crate::BitReader;
#[doc = "Field `FPRGM` writer - Flash program"]
pub type FPRGM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SECERS` reader - Sector erase"]
pub type SECERS_R = crate::BitReader;
#[doc = "Field `SECERS` writer - Sector erase"]
pub type SECERS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BANKERS` reader - Bank erase"]
pub type BANKERS_R = crate::BitReader;
#[doc = "Field `BANKERS` writer - Bank erase"]
pub type BANKERS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USDPRGM` reader - User system data program"]
pub type USDPRGM_R = crate::BitReader;
#[doc = "Field `USDPRGM` writer - User system data program"]
pub type USDPRGM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USDERS` reader - User system data erase"]
pub type USDERS_R = crate::BitReader;
#[doc = "Field `USDERS` writer - User system data erase"]
pub type USDERS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ERSTR` reader - Erasing start"]
pub type ERSTR_R = crate::BitReader;
#[doc = "Field `ERSTR` writer - Erasing start"]
pub type ERSTR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OPLK` reader - Operation lock"]
pub type OPLK_R = crate::BitReader;
#[doc = "Field `OPLK` writer - Operation lock"]
pub type OPLK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USDULKS` reader - User system data unlock success"]
pub type USDULKS_R = crate::BitReader;
#[doc = "Field `USDULKS` writer - User system data unlock success"]
pub type USDULKS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ERRIE` reader - Error interrupt enable"]
pub type ERRIE_R = crate::BitReader;
#[doc = "Field `ERRIE` writer - Error interrupt enable"]
pub type ERRIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ODFIE` reader - Operation done flag interrupt enable"]
pub type ODFIE_R = crate::BitReader;
#[doc = "Field `ODFIE` writer - Operation done flag interrupt enable"]
pub type ODFIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FAP_HL_DIS` reader - FAP high level disable"]
pub type FAP_HL_DIS_R = crate::BitReader;
#[doc = "Field `FAP_HL_DIS` writer - FAP high level disable"]
pub type FAP_HL_DIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LPMEN` reader - Low power mode enable"]
pub type LPMEN_R = crate::BitReader;
#[doc = "Field `LPMEN` writer - Low power mode enable"]
pub type LPMEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Flash program"]
    #[inline(always)]
    pub fn fprgm(&self) -> FPRGM_R {
        FPRGM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Sector erase"]
    #[inline(always)]
    pub fn secers(&self) -> SECERS_R {
        SECERS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Bank erase"]
    #[inline(always)]
    pub fn bankers(&self) -> BANKERS_R {
        BANKERS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - User system data program"]
    #[inline(always)]
    pub fn usdprgm(&self) -> USDPRGM_R {
        USDPRGM_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - User system data erase"]
    #[inline(always)]
    pub fn usders(&self) -> USDERS_R {
        USDERS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Erasing start"]
    #[inline(always)]
    pub fn erstr(&self) -> ERSTR_R {
        ERSTR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Operation lock"]
    #[inline(always)]
    pub fn oplk(&self) -> OPLK_R {
        OPLK_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - User system data unlock success"]
    #[inline(always)]
    pub fn usdulks(&self) -> USDULKS_R {
        USDULKS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Error interrupt enable"]
    #[inline(always)]
    pub fn errie(&self) -> ERRIE_R {
        ERRIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - Operation done flag interrupt enable"]
    #[inline(always)]
    pub fn odfie(&self) -> ODFIE_R {
        ODFIE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - FAP high level disable"]
    #[inline(always)]
    pub fn fap_hl_dis(&self) -> FAP_HL_DIS_R {
        FAP_HL_DIS_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Low power mode enable"]
    #[inline(always)]
    pub fn lpmen(&self) -> LPMEN_R {
        LPMEN_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Flash program"]
    #[inline(always)]
    #[must_use]
    pub fn fprgm(&mut self) -> FPRGM_W<CTRL_SPEC, 0> {
        FPRGM_W::new(self)
    }
    #[doc = "Bit 1 - Sector erase"]
    #[inline(always)]
    #[must_use]
    pub fn secers(&mut self) -> SECERS_W<CTRL_SPEC, 1> {
        SECERS_W::new(self)
    }
    #[doc = "Bit 2 - Bank erase"]
    #[inline(always)]
    #[must_use]
    pub fn bankers(&mut self) -> BANKERS_W<CTRL_SPEC, 2> {
        BANKERS_W::new(self)
    }
    #[doc = "Bit 4 - User system data program"]
    #[inline(always)]
    #[must_use]
    pub fn usdprgm(&mut self) -> USDPRGM_W<CTRL_SPEC, 4> {
        USDPRGM_W::new(self)
    }
    #[doc = "Bit 5 - User system data erase"]
    #[inline(always)]
    #[must_use]
    pub fn usders(&mut self) -> USDERS_W<CTRL_SPEC, 5> {
        USDERS_W::new(self)
    }
    #[doc = "Bit 6 - Erasing start"]
    #[inline(always)]
    #[must_use]
    pub fn erstr(&mut self) -> ERSTR_W<CTRL_SPEC, 6> {
        ERSTR_W::new(self)
    }
    #[doc = "Bit 7 - Operation lock"]
    #[inline(always)]
    #[must_use]
    pub fn oplk(&mut self) -> OPLK_W<CTRL_SPEC, 7> {
        OPLK_W::new(self)
    }
    #[doc = "Bit 9 - User system data unlock success"]
    #[inline(always)]
    #[must_use]
    pub fn usdulks(&mut self) -> USDULKS_W<CTRL_SPEC, 9> {
        USDULKS_W::new(self)
    }
    #[doc = "Bit 10 - Error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn errie(&mut self) -> ERRIE_W<CTRL_SPEC, 10> {
        ERRIE_W::new(self)
    }
    #[doc = "Bit 12 - Operation done flag interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn odfie(&mut self) -> ODFIE_W<CTRL_SPEC, 12> {
        ODFIE_W::new(self)
    }
    #[doc = "Bit 16 - FAP high level disable"]
    #[inline(always)]
    #[must_use]
    pub fn fap_hl_dis(&mut self) -> FAP_HL_DIS_W<CTRL_SPEC, 16> {
        FAP_HL_DIS_W::new(self)
    }
    #[doc = "Bit 17 - Low power mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn lpmen(&mut self) -> LPMEN_W<CTRL_SPEC, 17> {
        LPMEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL to value 0x0002_0080"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0002_0080;
}
