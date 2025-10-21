#[doc = "Register `CTRL2` reader"]
pub type R = crate::R<CTRL2_SPEC>;
#[doc = "Register `CTRL2` writer"]
pub type W = crate::W<CTRL2_SPEC>;
#[doc = "Field `DMAREN` reader - DMA receive enable"]
pub type DMAREN_R = crate::BitReader;
#[doc = "Field `DMAREN` writer - DMA receive enable"]
pub type DMAREN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMATEN` reader - DMA transmit enable"]
pub type DMATEN_R = crate::BitReader;
#[doc = "Field `DMATEN` writer - DMA transmit enable"]
pub type DMATEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HWCSOE` reader - Hardware CS output enable"]
pub type HWCSOE_R = crate::BitReader;
#[doc = "Field `HWCSOE` writer - Hardware CS output enable"]
pub type HWCSOE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERRIE` reader - Error interrupt enable"]
pub type ERRIE_R = crate::BitReader;
#[doc = "Field `ERRIE` writer - Error interrupt enable"]
pub type ERRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDBFIE` reader - Receive data buffer full interrupt enable"]
pub type RDBFIE_R = crate::BitReader;
#[doc = "Field `RDBFIE` writer - Receive data buffer full interrupt enable"]
pub type RDBFIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TDBEIE` reader - Transmit data buffer empty interrupt enable"]
pub type TDBEIE_R = crate::BitReader;
#[doc = "Field `TDBEIE` writer - Transmit data buffer empty interrupt enable"]
pub type TDBEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MDIV3` reader - Master clock frequency division bit3"]
pub type MDIV3_R = crate::BitReader;
#[doc = "Field `MDIV3` writer - Master clock frequency division bit3"]
pub type MDIV3_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DMA receive enable"]
    #[inline(always)]
    pub fn dmaren(&self) -> DMAREN_R {
        DMAREN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA transmit enable"]
    #[inline(always)]
    pub fn dmaten(&self) -> DMATEN_R {
        DMATEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Hardware CS output enable"]
    #[inline(always)]
    pub fn hwcsoe(&self) -> HWCSOE_R {
        HWCSOE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - Error interrupt enable"]
    #[inline(always)]
    pub fn errie(&self) -> ERRIE_R {
        ERRIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Receive data buffer full interrupt enable"]
    #[inline(always)]
    pub fn rdbfie(&self) -> RDBFIE_R {
        RDBFIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transmit data buffer empty interrupt enable"]
    #[inline(always)]
    pub fn tdbeie(&self) -> TDBEIE_R {
        TDBEIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Master clock frequency division bit3"]
    #[inline(always)]
    pub fn mdiv3(&self) -> MDIV3_R {
        MDIV3_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL2")
            .field("mdiv3", &self.mdiv3())
            .field("tdbeie", &self.tdbeie())
            .field("rdbfie", &self.rdbfie())
            .field("errie", &self.errie())
            .field("hwcsoe", &self.hwcsoe())
            .field("dmaten", &self.dmaten())
            .field("dmaren", &self.dmaren())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - DMA receive enable"]
    #[inline(always)]
    pub fn dmaren(&mut self) -> DMAREN_W<'_, CTRL2_SPEC> {
        DMAREN_W::new(self, 0)
    }
    #[doc = "Bit 1 - DMA transmit enable"]
    #[inline(always)]
    pub fn dmaten(&mut self) -> DMATEN_W<'_, CTRL2_SPEC> {
        DMATEN_W::new(self, 1)
    }
    #[doc = "Bit 2 - Hardware CS output enable"]
    #[inline(always)]
    pub fn hwcsoe(&mut self) -> HWCSOE_W<'_, CTRL2_SPEC> {
        HWCSOE_W::new(self, 2)
    }
    #[doc = "Bit 5 - Error interrupt enable"]
    #[inline(always)]
    pub fn errie(&mut self) -> ERRIE_W<'_, CTRL2_SPEC> {
        ERRIE_W::new(self, 5)
    }
    #[doc = "Bit 6 - Receive data buffer full interrupt enable"]
    #[inline(always)]
    pub fn rdbfie(&mut self) -> RDBFIE_W<'_, CTRL2_SPEC> {
        RDBFIE_W::new(self, 6)
    }
    #[doc = "Bit 7 - Transmit data buffer empty interrupt enable"]
    #[inline(always)]
    pub fn tdbeie(&mut self) -> TDBEIE_W<'_, CTRL2_SPEC> {
        TDBEIE_W::new(self, 7)
    }
    #[doc = "Bit 8 - Master clock frequency division bit3"]
    #[inline(always)]
    pub fn mdiv3(&mut self) -> MDIV3_W<'_, CTRL2_SPEC> {
        MDIV3_W::new(self, 8)
    }
}
#[doc = "control register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL2_SPEC;
impl crate::RegisterSpec for CTRL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl2::R`](R) reader structure"]
impl crate::Readable for CTRL2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl2::W`](W) writer structure"]
impl crate::Writable for CTRL2_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRL2 to value 0"]
impl crate::Resettable for CTRL2_SPEC {}
