#[doc = "Register `CTRL2` reader"]
pub type R = crate::R<CTRL2_SPEC>;
#[doc = "Register `CTRL2` writer"]
pub type W = crate::W<CTRL2_SPEC>;
#[doc = "Field `DMAREN` reader - DMA receive enable"]
pub type DMAREN_R = crate::BitReader;
#[doc = "Field `DMAREN` writer - DMA receive enable"]
pub type DMAREN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DMATEN` reader - DMA transmit enable"]
pub type DMATEN_R = crate::BitReader;
#[doc = "Field `DMATEN` writer - DMA transmit enable"]
pub type DMATEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ERRIE` reader - Error interrupt enable"]
pub type ERRIE_R = crate::BitReader;
#[doc = "Field `ERRIE` writer - Error interrupt enable"]
pub type ERRIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RDBFIE` reader - Receive data buffer full interrupt enable"]
pub type RDBFIE_R = crate::BitReader;
#[doc = "Field `RDBFIE` writer - Receive data buffer full interrupt enable"]
pub type RDBFIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TDBEIE` reader - Transmit data buffer empty interrupt enable"]
pub type TDBEIE_R = crate::BitReader;
#[doc = "Field `TDBEIE` writer - Transmit data buffer empty interrupt enable"]
pub type TDBEIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
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
}
impl W {
    #[doc = "Bit 0 - DMA receive enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmaren(&mut self) -> DMAREN_W<CTRL2_SPEC, 0> {
        DMAREN_W::new(self)
    }
    #[doc = "Bit 1 - DMA transmit enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmaten(&mut self) -> DMATEN_W<CTRL2_SPEC, 1> {
        DMATEN_W::new(self)
    }
    #[doc = "Bit 5 - Error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn errie(&mut self) -> ERRIE_W<CTRL2_SPEC, 5> {
        ERRIE_W::new(self)
    }
    #[doc = "Bit 6 - Receive data buffer full interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rdbfie(&mut self) -> RDBFIE_W<CTRL2_SPEC, 6> {
        RDBFIE_W::new(self)
    }
    #[doc = "Bit 7 - Transmit data buffer empty interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tdbeie(&mut self) -> TDBEIE_W<CTRL2_SPEC, 7> {
        TDBEIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets CTRL2 to value 0"]
impl crate::Resettable for CTRL2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}