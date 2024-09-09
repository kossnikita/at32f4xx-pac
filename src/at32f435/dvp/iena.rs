#[doc = "Register `IENA` reader"]
pub type R = crate::R<IENA_SPEC>;
#[doc = "Register `IENA` writer"]
pub type W = crate::W<IENA_SPEC>;
#[doc = "Field `CFDIE` reader - Capture frame done interrupt enable"]
pub type CFDIE_R = crate::BitReader;
#[doc = "Field `CFDIE` writer - Capture frame done interrupt enable"]
pub type CFDIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVRIE` reader - Data FIFO overrun interrupt enable"]
pub type OVRIE_R = crate::BitReader;
#[doc = "Field `OVRIE` writer - Data FIFO overrun interrupt enable"]
pub type OVRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ESEIE` reader - Embedded synchronization error interrupt enable"]
pub type ESEIE_R = crate::BitReader;
#[doc = "Field `ESEIE` writer - Embedded synchronization error interrupt enable"]
pub type ESEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VSIE` reader - Vertical synchronization interrupt enablee"]
pub type VSIE_R = crate::BitReader;
#[doc = "Field `VSIE` writer - Vertical synchronization interrupt enablee"]
pub type VSIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSIE` reader - Horizontal synchronization interrupt enable"]
pub type HSIE_R = crate::BitReader;
#[doc = "Field `HSIE` writer - Horizontal synchronization interrupt enable"]
pub type HSIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Capture frame done interrupt enable"]
    #[inline(always)]
    pub fn cfdie(&self) -> CFDIE_R {
        CFDIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Data FIFO overrun interrupt enable"]
    #[inline(always)]
    pub fn ovrie(&self) -> OVRIE_R {
        OVRIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Embedded synchronization error interrupt enable"]
    #[inline(always)]
    pub fn eseie(&self) -> ESEIE_R {
        ESEIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Vertical synchronization interrupt enablee"]
    #[inline(always)]
    pub fn vsie(&self) -> VSIE_R {
        VSIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Horizontal synchronization interrupt enable"]
    #[inline(always)]
    pub fn hsie(&self) -> HSIE_R {
        HSIE_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IENA")
            .field("hsie", &self.hsie())
            .field("vsie", &self.vsie())
            .field("eseie", &self.eseie())
            .field("ovrie", &self.ovrie())
            .field("cfdie", &self.cfdie())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Capture frame done interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn cfdie(&mut self) -> CFDIE_W<IENA_SPEC> {
        CFDIE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Data FIFO overrun interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ovrie(&mut self) -> OVRIE_W<IENA_SPEC> {
        OVRIE_W::new(self, 1)
    }
    #[doc = "Bit 2 - Embedded synchronization error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn eseie(&mut self) -> ESEIE_W<IENA_SPEC> {
        ESEIE_W::new(self, 2)
    }
    #[doc = "Bit 3 - Vertical synchronization interrupt enablee"]
    #[inline(always)]
    #[must_use]
    pub fn vsie(&mut self) -> VSIE_W<IENA_SPEC> {
        VSIE_W::new(self, 3)
    }
    #[doc = "Bit 4 - Horizontal synchronization interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn hsie(&mut self) -> HSIE_W<IENA_SPEC> {
        HSIE_W::new(self, 4)
    }
}
#[doc = "interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`iena::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iena::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IENA_SPEC;
impl crate::RegisterSpec for IENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iena::R`](R) reader structure"]
impl crate::Readable for IENA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`iena::W`](W) writer structure"]
impl crate::Writable for IENA_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IENA to value 0"]
impl crate::Resettable for IENA_SPEC {
    const RESET_VALUE: u32 = 0;
}
