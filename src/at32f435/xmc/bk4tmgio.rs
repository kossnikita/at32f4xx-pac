#[doc = "Register `BK4TMGIO` reader"]
pub type R = crate::R<BK4TMGIO_SPEC>;
#[doc = "Register `BK4TMGIO` writer"]
pub type W = crate::W<BK4TMGIO_SPEC>;
#[doc = "Field `IOST` reader - STP"]
pub type IOST_R = crate::FieldReader;
#[doc = "Field `IOST` writer - STP"]
pub type IOST_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `IOWT` reader - OP"]
pub type IOWT_R = crate::FieldReader;
#[doc = "Field `IOWT` writer - OP"]
pub type IOWT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `IOHT` reader - HLD"]
pub type IOHT_R = crate::FieldReader;
#[doc = "Field `IOHT` writer - HLD"]
pub type IOHT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `IODHIZT` reader - WRSTP"]
pub type IODHIZT_R = crate::FieldReader;
#[doc = "Field `IODHIZT` writer - WRSTP"]
pub type IODHIZT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
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
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BK4TMGIO")
            .field("iodhizt", &self.iodhizt())
            .field("ioht", &self.ioht())
            .field("iowt", &self.iowt())
            .field("iost", &self.iost())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - STP"]
    #[inline(always)]
    #[must_use]
    pub fn iost(&mut self) -> IOST_W<BK4TMGIO_SPEC> {
        IOST_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - OP"]
    #[inline(always)]
    #[must_use]
    pub fn iowt(&mut self) -> IOWT_W<BK4TMGIO_SPEC> {
        IOWT_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - HLD"]
    #[inline(always)]
    #[must_use]
    pub fn ioht(&mut self) -> IOHT_W<BK4TMGIO_SPEC> {
        IOHT_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - WRSTP"]
    #[inline(always)]
    #[must_use]
    pub fn iodhizt(&mut self) -> IODHIZT_W<BK4TMGIO_SPEC> {
        IODHIZT_W::new(self, 24)
    }
}
#[doc = "I/O space timing register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`bk4tmgio::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bk4tmgio::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BK4TMGIO_SPEC;
impl crate::RegisterSpec for BK4TMGIO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bk4tmgio::R`](R) reader structure"]
impl crate::Readable for BK4TMGIO_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bk4tmgio::W`](W) writer structure"]
impl crate::Writable for BK4TMGIO_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BK4TMGIO to value 0xfcfc_fcfc"]
impl crate::Resettable for BK4TMGIO_SPEC {
    const RESET_VALUE: u32 = 0xfcfc_fcfc;
}
