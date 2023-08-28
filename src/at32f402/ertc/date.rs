#[doc = "Register `DATE` reader"]
pub type R = crate::R<DATE_SPEC>;
#[doc = "Register `DATE` writer"]
pub type W = crate::W<DATE_SPEC>;
#[doc = "Field `DU` reader - Date units"]
pub type DU_R = crate::FieldReader;
#[doc = "Field `DU` writer - Date units"]
pub type DU_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `DT` reader - Date tens"]
pub type DT_R = crate::FieldReader;
#[doc = "Field `DT` writer - Date tens"]
pub type DT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `MU` reader - Month units"]
pub type MU_R = crate::FieldReader;
#[doc = "Field `MU` writer - Month units"]
pub type MU_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `MT` reader - Month tens"]
pub type MT_R = crate::BitReader;
#[doc = "Field `MT` writer - Month tens"]
pub type MT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WK` reader - Week"]
pub type WK_R = crate::FieldReader;
#[doc = "Field `WK` writer - Week"]
pub type WK_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `YU` reader - Year units"]
pub type YU_R = crate::FieldReader;
#[doc = "Field `YU` writer - Year units"]
pub type YU_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `YT` reader - Year tens"]
pub type YT_R = crate::FieldReader;
#[doc = "Field `YT` writer - Year tens"]
pub type YT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
impl R {
    #[doc = "Bits 0:3 - Date units"]
    #[inline(always)]
    pub fn du(&self) -> DU_R {
        DU_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - Date tens"]
    #[inline(always)]
    pub fn dt(&self) -> DT_R {
        DT_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:11 - Month units"]
    #[inline(always)]
    pub fn mu(&self) -> MU_R {
        MU_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - Month tens"]
    #[inline(always)]
    pub fn mt(&self) -> MT_R {
        MT_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:15 - Week"]
    #[inline(always)]
    pub fn wk(&self) -> WK_R {
        WK_R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:19 - Year units"]
    #[inline(always)]
    pub fn yu(&self) -> YU_R {
        YU_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Year tens"]
    #[inline(always)]
    pub fn yt(&self) -> YT_R {
        YT_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Date units"]
    #[inline(always)]
    #[must_use]
    pub fn du(&mut self) -> DU_W<DATE_SPEC, 0> {
        DU_W::new(self)
    }
    #[doc = "Bits 4:5 - Date tens"]
    #[inline(always)]
    #[must_use]
    pub fn dt(&mut self) -> DT_W<DATE_SPEC, 4> {
        DT_W::new(self)
    }
    #[doc = "Bits 8:11 - Month units"]
    #[inline(always)]
    #[must_use]
    pub fn mu(&mut self) -> MU_W<DATE_SPEC, 8> {
        MU_W::new(self)
    }
    #[doc = "Bit 12 - Month tens"]
    #[inline(always)]
    #[must_use]
    pub fn mt(&mut self) -> MT_W<DATE_SPEC, 12> {
        MT_W::new(self)
    }
    #[doc = "Bits 13:15 - Week"]
    #[inline(always)]
    #[must_use]
    pub fn wk(&mut self) -> WK_W<DATE_SPEC, 13> {
        WK_W::new(self)
    }
    #[doc = "Bits 16:19 - Year units"]
    #[inline(always)]
    #[must_use]
    pub fn yu(&mut self) -> YU_W<DATE_SPEC, 16> {
        YU_W::new(self)
    }
    #[doc = "Bits 20:23 - Year tens"]
    #[inline(always)]
    #[must_use]
    pub fn yt(&mut self) -> YT_W<DATE_SPEC, 20> {
        YT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "date register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`date::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DATE_SPEC;
impl crate::RegisterSpec for DATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`date::R`](R) reader structure"]
impl crate::Readable for DATE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`date::W`](W) writer structure"]
impl crate::Writable for DATE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DATE to value 0x2101"]
impl crate::Resettable for DATE_SPEC {
    const RESET_VALUE: Self::Ux = 0x2101;
}
