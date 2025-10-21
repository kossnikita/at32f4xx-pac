#[doc = "Register `S2FCTRL` reader"]
pub type R = crate::R<S2FCTRL_SPEC>;
#[doc = "Register `S2FCTRL` writer"]
pub type W = crate::W<S2FCTRL_SPEC>;
#[doc = "Field `FTHSEL` reader - FIFO threshold selection"]
pub type FTHSEL_R = crate::FieldReader;
#[doc = "Field `FTHSEL` writer - FIFO threshold selection"]
pub type FTHSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FEN` reader - FIFO mode enable"]
pub type FEN_R = crate::BitReader;
#[doc = "Field `FEN` writer - FIFO mode enable"]
pub type FEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FSTS` reader - FIFO status"]
pub type FSTS_R = crate::FieldReader;
#[doc = "Field `FERRIEN` reader - FIFO error interrupt enable"]
pub type FERRIEN_R = crate::BitReader;
#[doc = "Field `FERRIEN` writer - FIFO error interrupt enable"]
pub type FERRIEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - FIFO threshold selection"]
    #[inline(always)]
    pub fn fthsel(&self) -> FTHSEL_R {
        FTHSEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - FIFO mode enable"]
    #[inline(always)]
    pub fn fen(&self) -> FEN_R {
        FEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:5 - FIFO status"]
    #[inline(always)]
    pub fn fsts(&self) -> FSTS_R {
        FSTS_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bit 7 - FIFO error interrupt enable"]
    #[inline(always)]
    pub fn ferrien(&self) -> FERRIEN_R {
        FERRIEN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("S2FCTRL")
            .field("ferrien", &self.ferrien())
            .field("fsts", &self.fsts())
            .field("fen", &self.fen())
            .field("fthsel", &self.fthsel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - FIFO threshold selection"]
    #[inline(always)]
    pub fn fthsel(&mut self) -> FTHSEL_W<'_, S2FCTRL_SPEC> {
        FTHSEL_W::new(self, 0)
    }
    #[doc = "Bit 2 - FIFO mode enable"]
    #[inline(always)]
    pub fn fen(&mut self) -> FEN_W<'_, S2FCTRL_SPEC> {
        FEN_W::new(self, 2)
    }
    #[doc = "Bit 7 - FIFO error interrupt enable"]
    #[inline(always)]
    pub fn ferrien(&mut self) -> FERRIEN_W<'_, S2FCTRL_SPEC> {
        FERRIEN_W::new(self, 7)
    }
}
#[doc = "stream 2 FIFO control register\n\nYou can [`read`](crate::Reg::read) this register and get [`s2fctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s2fctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct S2FCTRL_SPEC;
impl crate::RegisterSpec for S2FCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`s2fctrl::R`](R) reader structure"]
impl crate::Readable for S2FCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`s2fctrl::W`](W) writer structure"]
impl crate::Writable for S2FCTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets S2FCTRL to value 0x21"]
impl crate::Resettable for S2FCTRL_SPEC {
    const RESET_VALUE: u32 = 0x21;
}
