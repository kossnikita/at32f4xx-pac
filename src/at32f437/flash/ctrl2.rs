#[doc = "Register `CTRL2` reader"]
pub type R = crate::R<CTRL2_SPEC>;
#[doc = "Register `CTRL2` writer"]
pub type W = crate::W<CTRL2_SPEC>;
#[doc = "Field `FPRGM` reader - Flash program"]
pub type FPRGM_R = crate::BitReader;
#[doc = "Field `FPRGM` writer - Flash program"]
pub type FPRGM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SECERS` reader - Sector erase"]
pub type SECERS_R = crate::BitReader;
#[doc = "Field `SECERS` writer - Sector erase"]
pub type SECERS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BANKERS` reader - Bank erase"]
pub type BANKERS_R = crate::BitReader;
#[doc = "Field `BANKERS` writer - Bank erase"]
pub type BANKERS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLKERS` reader - Block erase"]
pub type BLKERS_R = crate::BitReader;
#[doc = "Field `BLKERS` writer - Block erase"]
pub type BLKERS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERSTR` reader - Erasing start"]
pub type ERSTR_R = crate::BitReader;
#[doc = "Field `ERSTR` writer - Erasing start"]
pub type ERSTR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPLK` reader - Operation lock"]
pub type OPLK_R = crate::BitReader;
#[doc = "Field `OPLK` writer - Operation lock"]
pub type OPLK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERRIE` reader - Error interrupt enable"]
pub type ERRIE_R = crate::BitReader;
#[doc = "Field `ERRIE` writer - Error interrupt enable"]
pub type ERRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ODFIE` reader - Operation done flag interrupt enable"]
pub type ODFIE_R = crate::BitReader;
#[doc = "Field `ODFIE` writer - Operation done flag interrupt enable"]
pub type ODFIE_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 3 - Block erase"]
    #[inline(always)]
    pub fn blkers(&self) -> BLKERS_R {
        BLKERS_R::new(((self.bits >> 3) & 1) != 0)
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
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL2")
            .field("fprgm", &format_args!("{}", self.fprgm().bit()))
            .field("secers", &format_args!("{}", self.secers().bit()))
            .field("bankers", &format_args!("{}", self.bankers().bit()))
            .field("blkers", &format_args!("{}", self.blkers().bit()))
            .field("erstr", &format_args!("{}", self.erstr().bit()))
            .field("oplk", &format_args!("{}", self.oplk().bit()))
            .field("errie", &format_args!("{}", self.errie().bit()))
            .field("odfie", &format_args!("{}", self.odfie().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<CTRL2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Flash program"]
    #[inline(always)]
    #[must_use]
    pub fn fprgm(&mut self) -> FPRGM_W<CTRL2_SPEC> {
        FPRGM_W::new(self, 0)
    }
    #[doc = "Bit 1 - Sector erase"]
    #[inline(always)]
    #[must_use]
    pub fn secers(&mut self) -> SECERS_W<CTRL2_SPEC> {
        SECERS_W::new(self, 1)
    }
    #[doc = "Bit 2 - Bank erase"]
    #[inline(always)]
    #[must_use]
    pub fn bankers(&mut self) -> BANKERS_W<CTRL2_SPEC> {
        BANKERS_W::new(self, 2)
    }
    #[doc = "Bit 3 - Block erase"]
    #[inline(always)]
    #[must_use]
    pub fn blkers(&mut self) -> BLKERS_W<CTRL2_SPEC> {
        BLKERS_W::new(self, 3)
    }
    #[doc = "Bit 6 - Erasing start"]
    #[inline(always)]
    #[must_use]
    pub fn erstr(&mut self) -> ERSTR_W<CTRL2_SPEC> {
        ERSTR_W::new(self, 6)
    }
    #[doc = "Bit 7 - Operation lock"]
    #[inline(always)]
    #[must_use]
    pub fn oplk(&mut self) -> OPLK_W<CTRL2_SPEC> {
        OPLK_W::new(self, 7)
    }
    #[doc = "Bit 10 - Error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn errie(&mut self) -> ERRIE_W<CTRL2_SPEC> {
        ERRIE_W::new(self, 10)
    }
    #[doc = "Bit 12 - Operation done flag interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn odfie(&mut self) -> ODFIE_W<CTRL2_SPEC> {
        ODFIE_W::new(self, 12)
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
#[doc = "Control 2 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL2_SPEC;
impl crate::RegisterSpec for CTRL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl2::R`](R) reader structure"]
impl crate::Readable for CTRL2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl2::W`](W) writer structure"]
impl crate::Writable for CTRL2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL2 to value 0x80"]
impl crate::Resettable for CTRL2_SPEC {
    const RESET_VALUE: Self::Ux = 0x80;
}
