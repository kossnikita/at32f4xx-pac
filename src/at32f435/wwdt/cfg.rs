#[doc = "Register `CFG` reader"]
pub type R = crate::R<CFG_SPEC>;
#[doc = "Register `CFG` writer"]
pub type W = crate::W<CFG_SPEC>;
#[doc = "Field `WIN` reader - Window value"]
pub type WIN_R = crate::FieldReader;
#[doc = "Field `WIN` writer - Window value"]
pub type WIN_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
#[doc = "Field `DIV` reader - Clock division value"]
pub type DIV_R = crate::FieldReader;
#[doc = "Field `DIV` writer - Clock division value"]
pub type DIV_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `RLDIEN` reader - Reload counter interrupt"]
pub type RLDIEN_R = crate::BitReader;
#[doc = "Field `RLDIEN` writer - Reload counter interrupt"]
pub type RLDIEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:6 - Window value"]
    #[inline(always)]
    pub fn win(&self) -> WIN_R {
        WIN_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 7:8 - Clock division value"]
    #[inline(always)]
    pub fn div(&self) -> DIV_R {
        DIV_R::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bit 9 - Reload counter interrupt"]
    #[inline(always)]
    pub fn rldien(&self) -> RLDIEN_R {
        RLDIEN_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Window value"]
    #[inline(always)]
    #[must_use]
    pub fn win(&mut self) -> WIN_W<CFG_SPEC, 0> {
        WIN_W::new(self)
    }
    #[doc = "Bits 7:8 - Clock division value"]
    #[inline(always)]
    #[must_use]
    pub fn div(&mut self) -> DIV_W<CFG_SPEC, 7> {
        DIV_W::new(self)
    }
    #[doc = "Bit 9 - Reload counter interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn rldien(&mut self) -> RLDIEN_W<CFG_SPEC, 9> {
        RLDIEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFG_SPEC;
impl crate::RegisterSpec for CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg::R`](R) reader structure"]
impl crate::Readable for CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cfg::W`](W) writer structure"]
impl crate::Writable for CFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFG to value 0x7f"]
impl crate::Resettable for CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0x7f;
}
