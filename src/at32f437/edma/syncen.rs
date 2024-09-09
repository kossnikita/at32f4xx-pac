#[doc = "Register `SYNCEN` reader"]
pub type R = crate::R<SYNCEN_SPEC>;
#[doc = "Register `SYNCEN` writer"]
pub type W = crate::W<SYNCEN_SPEC>;
#[doc = "Field `S1SYNC` reader - Stream 1 sync enable"]
pub type S1SYNC_R = crate::BitReader;
#[doc = "Field `S1SYNC` writer - Stream 1 sync enable"]
pub type S1SYNC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S2SYNC` reader - Stream 2 sync enable"]
pub type S2SYNC_R = crate::BitReader;
#[doc = "Field `S2SYNC` writer - Stream 2 sync enable"]
pub type S2SYNC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S3SYNC` reader - Stream 3 sync enable"]
pub type S3SYNC_R = crate::BitReader;
#[doc = "Field `S3SYNC` writer - Stream 3 sync enable"]
pub type S3SYNC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S4SYNC` reader - Stream 4 sync enable"]
pub type S4SYNC_R = crate::BitReader;
#[doc = "Field `S4SYNC` writer - Stream 4 sync enable"]
pub type S4SYNC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S5SYNC` reader - Stream 5 sync enable"]
pub type S5SYNC_R = crate::BitReader;
#[doc = "Field `S5SYNC` writer - Stream 5 sync enable"]
pub type S5SYNC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S6SYNC` reader - Stream 6 sync enable"]
pub type S6SYNC_R = crate::BitReader;
#[doc = "Field `S6SYNC` writer - Stream 6 sync enable"]
pub type S6SYNC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S7SYNC` reader - Stream 7 sync enable"]
pub type S7SYNC_R = crate::BitReader;
#[doc = "Field `S7SYNC` writer - Stream 7 sync enable"]
pub type S7SYNC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S8SYNC` reader - Stream 8 sync enable"]
pub type S8SYNC_R = crate::BitReader;
#[doc = "Field `S8SYNC` writer - Stream 8 sync enable"]
pub type S8SYNC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Stream 1 sync enable"]
    #[inline(always)]
    pub fn s1sync(&self) -> S1SYNC_R {
        S1SYNC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Stream 2 sync enable"]
    #[inline(always)]
    pub fn s2sync(&self) -> S2SYNC_R {
        S2SYNC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Stream 3 sync enable"]
    #[inline(always)]
    pub fn s3sync(&self) -> S3SYNC_R {
        S3SYNC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Stream 4 sync enable"]
    #[inline(always)]
    pub fn s4sync(&self) -> S4SYNC_R {
        S4SYNC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Stream 5 sync enable"]
    #[inline(always)]
    pub fn s5sync(&self) -> S5SYNC_R {
        S5SYNC_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Stream 6 sync enable"]
    #[inline(always)]
    pub fn s6sync(&self) -> S6SYNC_R {
        S6SYNC_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Stream 7 sync enable"]
    #[inline(always)]
    pub fn s7sync(&self) -> S7SYNC_R {
        S7SYNC_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Stream 8 sync enable"]
    #[inline(always)]
    pub fn s8sync(&self) -> S8SYNC_R {
        S8SYNC_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYNCEN")
            .field("s1sync", &self.s1sync())
            .field("s2sync", &self.s2sync())
            .field("s3sync", &self.s3sync())
            .field("s4sync", &self.s4sync())
            .field("s5sync", &self.s5sync())
            .field("s6sync", &self.s6sync())
            .field("s7sync", &self.s7sync())
            .field("s8sync", &self.s8sync())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Stream 1 sync enable"]
    #[inline(always)]
    #[must_use]
    pub fn s1sync(&mut self) -> S1SYNC_W<SYNCEN_SPEC> {
        S1SYNC_W::new(self, 0)
    }
    #[doc = "Bit 1 - Stream 2 sync enable"]
    #[inline(always)]
    #[must_use]
    pub fn s2sync(&mut self) -> S2SYNC_W<SYNCEN_SPEC> {
        S2SYNC_W::new(self, 1)
    }
    #[doc = "Bit 2 - Stream 3 sync enable"]
    #[inline(always)]
    #[must_use]
    pub fn s3sync(&mut self) -> S3SYNC_W<SYNCEN_SPEC> {
        S3SYNC_W::new(self, 2)
    }
    #[doc = "Bit 3 - Stream 4 sync enable"]
    #[inline(always)]
    #[must_use]
    pub fn s4sync(&mut self) -> S4SYNC_W<SYNCEN_SPEC> {
        S4SYNC_W::new(self, 3)
    }
    #[doc = "Bit 4 - Stream 5 sync enable"]
    #[inline(always)]
    #[must_use]
    pub fn s5sync(&mut self) -> S5SYNC_W<SYNCEN_SPEC> {
        S5SYNC_W::new(self, 4)
    }
    #[doc = "Bit 5 - Stream 6 sync enable"]
    #[inline(always)]
    #[must_use]
    pub fn s6sync(&mut self) -> S6SYNC_W<SYNCEN_SPEC> {
        S6SYNC_W::new(self, 5)
    }
    #[doc = "Bit 6 - Stream 7 sync enable"]
    #[inline(always)]
    #[must_use]
    pub fn s7sync(&mut self) -> S7SYNC_W<SYNCEN_SPEC> {
        S7SYNC_W::new(self, 6)
    }
    #[doc = "Bit 7 - Stream 8 sync enable"]
    #[inline(always)]
    #[must_use]
    pub fn s8sync(&mut self) -> S8SYNC_W<SYNCEN_SPEC> {
        S8SYNC_W::new(self, 7)
    }
}
#[doc = "Sync Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`syncen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syncen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYNCEN_SPEC;
impl crate::RegisterSpec for SYNCEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syncen::R`](R) reader structure"]
impl crate::Readable for SYNCEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`syncen::W`](W) writer structure"]
impl crate::Writable for SYNCEN_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SYNCEN to value 0"]
impl crate::Resettable for SYNCEN_SPEC {
    const RESET_VALUE: u32 = 0;
}
