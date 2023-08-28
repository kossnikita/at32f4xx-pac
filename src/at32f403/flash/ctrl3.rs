#[doc = "Register `CTRL3` reader"]
pub type R = crate::R<CTRL3_SPEC>;
#[doc = "Register `CTRL3` writer"]
pub type W = crate::W<CTRL3_SPEC>;
#[doc = "Field `FPRGM` reader - Flash program"]
pub type FPRGM_R = crate::BitReader;
#[doc = "Field `FPRGM` writer - Flash program"]
pub type FPRGM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SECERS` reader - Sector erase"]
pub type SECERS_R = crate::BitReader;
#[doc = "Field `SECERS` writer - Sector erase"]
pub type SECERS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CHPERS` reader - Chip erase"]
pub type CHPERS_R = crate::BitReader;
#[doc = "Field `CHPERS` writer - Chip erase"]
pub type CHPERS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ERSTR` reader - Erasing start"]
pub type ERSTR_R = crate::BitReader;
#[doc = "Field `ERSTR` writer - Erasing start"]
pub type ERSTR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OPLK` reader - Operation lock"]
pub type OPLK_R = crate::BitReader;
#[doc = "Field `OPLK` writer - Operation lock"]
pub type OPLK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ERRIE` reader - Error interrupt enable"]
pub type ERRIE_R = crate::BitReader;
#[doc = "Field `ERRIE` writer - Error interrupt enable"]
pub type ERRIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ODFIE` reader - Operation done flag interrupt enable"]
pub type ODFIE_R = crate::BitReader;
#[doc = "Field `ODFIE` writer - Operation done flag interrupt enable"]
pub type ODFIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
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
    #[doc = "Bit 2 - Chip erase"]
    #[inline(always)]
    pub fn chpers(&self) -> CHPERS_R {
        CHPERS_R::new(((self.bits >> 2) & 1) != 0)
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
}
impl W {
    #[doc = "Bit 0 - Flash program"]
    #[inline(always)]
    #[must_use]
    pub fn fprgm(&mut self) -> FPRGM_W<CTRL3_SPEC, 0> {
        FPRGM_W::new(self)
    }
    #[doc = "Bit 1 - Sector erase"]
    #[inline(always)]
    #[must_use]
    pub fn secers(&mut self) -> SECERS_W<CTRL3_SPEC, 1> {
        SECERS_W::new(self)
    }
    #[doc = "Bit 2 - Chip erase"]
    #[inline(always)]
    #[must_use]
    pub fn chpers(&mut self) -> CHPERS_W<CTRL3_SPEC, 2> {
        CHPERS_W::new(self)
    }
    #[doc = "Bit 6 - Erasing start"]
    #[inline(always)]
    #[must_use]
    pub fn erstr(&mut self) -> ERSTR_W<CTRL3_SPEC, 6> {
        ERSTR_W::new(self)
    }
    #[doc = "Bit 7 - Operation lock"]
    #[inline(always)]
    #[must_use]
    pub fn oplk(&mut self) -> OPLK_W<CTRL3_SPEC, 7> {
        OPLK_W::new(self)
    }
    #[doc = "Bit 10 - Error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn errie(&mut self) -> ERRIE_W<CTRL3_SPEC, 10> {
        ERRIE_W::new(self)
    }
    #[doc = "Bit 12 - Operation done flag interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn odfie(&mut self) -> ODFIE_W<CTRL3_SPEC, 12> {
        ODFIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Control 3 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL3_SPEC;
impl crate::RegisterSpec for CTRL3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl3::R`](R) reader structure"]
impl crate::Readable for CTRL3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl3::W`](W) writer structure"]
impl crate::Writable for CTRL3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL3 to value 0x80"]
impl crate::Resettable for CTRL3_SPEC {
    const RESET_VALUE: Self::Ux = 0x80;
}
