#[doc = "Register `BK4CTRL` reader"]
pub type R = crate::R<BK4CTRL_SPEC>;
#[doc = "Register `BK4CTRL` writer"]
pub type W = crate::W<BK4CTRL_SPEC>;
#[doc = "Field `NWEN` reader - Wait feature enable"]
pub type NWEN_R = crate::BitReader;
#[doc = "Field `NWEN` writer - Wait feature enable"]
pub type NWEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EN` reader - Memory bank enable"]
pub type EN_R = crate::BitReader;
#[doc = "Field `EN` writer - Memory bank enable"]
pub type EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 1 - Wait feature enable"]
    #[inline(always)]
    pub fn nwen(&self) -> NWEN_R {
        NWEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Memory bank enable"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BK4CTRL")
            .field("en", &format_args!("{}", self.en().bit()))
            .field("nwen", &format_args!("{}", self.nwen().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<BK4CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 1 - Wait feature enable"]
    #[inline(always)]
    #[must_use]
    pub fn nwen(&mut self) -> NWEN_W<BK4CTRL_SPEC, 1> {
        NWEN_W::new(self)
    }
    #[doc = "Bit 2 - Memory bank enable"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<BK4CTRL_SPEC, 2> {
        EN_W::new(self)
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
#[doc = "PC Card/NAND Flash control register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bk4ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bk4ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BK4CTRL_SPEC;
impl crate::RegisterSpec for BK4CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bk4ctrl::R`](R) reader structure"]
impl crate::Readable for BK4CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bk4ctrl::W`](W) writer structure"]
impl crate::Writable for BK4CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BK4CTRL to value 0x18"]
impl crate::Resettable for BK4CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x18;
}
