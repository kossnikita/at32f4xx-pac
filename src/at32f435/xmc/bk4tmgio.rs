#[doc = "Register `BK4TMGIO` reader"]
pub type R = crate::R<BK4TMGIO_SPEC>;
#[doc = "Register `BK4TMGIO` writer"]
pub type W = crate::W<BK4TMGIO_SPEC>;
#[doc = "Field `IOST` reader - STP"]
pub type IOST_R = crate::FieldReader;
#[doc = "Field `IOST` writer - STP"]
pub type IOST_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `IOWT` reader - OP"]
pub type IOWT_R = crate::FieldReader;
#[doc = "Field `IOWT` writer - OP"]
pub type IOWT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `IOHT` reader - HLD"]
pub type IOHT_R = crate::FieldReader;
#[doc = "Field `IOHT` writer - HLD"]
pub type IOHT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `IODHIZT` reader - WRSTP"]
pub type IODHIZT_R = crate::FieldReader;
#[doc = "Field `IODHIZT` writer - WRSTP"]
pub type IODHIZT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - STP"]
    #[inline(always)]
    pub fn iost(&self) -> IOST_R {
        IOST_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - OP"]
    #[inline(always)]
    pub fn iowt(&self) -> IOWT_R {
        IOWT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - HLD"]
    #[inline(always)]
    pub fn ioht(&self) -> IOHT_R {
        IOHT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - WRSTP"]
    #[inline(always)]
    pub fn iodhizt(&self) -> IODHIZT_R {
        IODHIZT_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - STP"]
    #[inline(always)]
    #[must_use]
    pub fn iost(&mut self) -> IOST_W<BK4TMGIO_SPEC, 0> {
        IOST_W::new(self)
    }
    #[doc = "Bits 8:15 - OP"]
    #[inline(always)]
    #[must_use]
    pub fn iowt(&mut self) -> IOWT_W<BK4TMGIO_SPEC, 8> {
        IOWT_W::new(self)
    }
    #[doc = "Bits 16:23 - HLD"]
    #[inline(always)]
    #[must_use]
    pub fn ioht(&mut self) -> IOHT_W<BK4TMGIO_SPEC, 16> {
        IOHT_W::new(self)
    }
    #[doc = "Bits 24:31 - WRSTP"]
    #[inline(always)]
    #[must_use]
    pub fn iodhizt(&mut self) -> IODHIZT_W<BK4TMGIO_SPEC, 24> {
        IODHIZT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "I/O space timing register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bk4tmgio::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bk4tmgio::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BK4TMGIO_SPEC;
impl crate::RegisterSpec for BK4TMGIO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bk4tmgio::R`](R) reader structure"]
impl crate::Readable for BK4TMGIO_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bk4tmgio::W`](W) writer structure"]
impl crate::Writable for BK4TMGIO_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BK4TMGIO to value 0xfcfc_fcfc"]
impl crate::Resettable for BK4TMGIO_SPEC {
    const RESET_VALUE: Self::Ux = 0xfcfc_fcfc;
}
