#[doc = "Register `CTRL1` reader"]
pub type R = crate::R<CTRL1_SPEC>;
#[doc = "Register `CTRL1` writer"]
pub type W = crate::W<CTRL1_SPEC>;
#[doc = "Field `SBF` reader - Send break frame"]
pub type SBF_R = crate::BitReader;
#[doc = "Field `SBF` writer - Send break frame"]
pub type SBF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RM` reader - Receiver mute"]
pub type RM_R = crate::BitReader;
#[doc = "Field `RM` writer - Receiver mute"]
pub type RM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REN` reader - Receiver enable"]
pub type REN_R = crate::BitReader;
#[doc = "Field `REN` writer - Receiver enable"]
pub type REN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEN` reader - Transmitter enable"]
pub type TEN_R = crate::BitReader;
#[doc = "Field `TEN` writer - Transmitter enable"]
pub type TEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDLEIEN` reader - IDLE interrupt enable"]
pub type IDLEIEN_R = crate::BitReader;
#[doc = "Field `IDLEIEN` writer - IDLE interrupt enable"]
pub type IDLEIEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDBFIEN` reader - RDBF interrupt enable"]
pub type RDBFIEN_R = crate::BitReader;
#[doc = "Field `RDBFIEN` writer - RDBF interrupt enable"]
pub type RDBFIEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TDCIEN` reader - TDC interrupt enable"]
pub type TDCIEN_R = crate::BitReader;
#[doc = "Field `TDCIEN` writer - TDC interrupt enable"]
pub type TDCIEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TDBEIEN` reader - TDBE interrupt enable"]
pub type TDBEIEN_R = crate::BitReader;
#[doc = "Field `TDBEIEN` writer - TDBE interrupt enable"]
pub type TDBEIEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PERRIEN` reader - PERR interrupt enable"]
pub type PERRIEN_R = crate::BitReader;
#[doc = "Field `PERRIEN` writer - PERR interrupt enable"]
pub type PERRIEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PSEL` reader - Parity selection"]
pub type PSEL_R = crate::BitReader;
#[doc = "Field `PSEL` writer - Parity selection"]
pub type PSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEN` reader - Parity enable"]
pub type PEN_R = crate::BitReader;
#[doc = "Field `PEN` writer - Parity enable"]
pub type PEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WUM` reader - Wake up mode"]
pub type WUM_R = crate::BitReader;
#[doc = "Field `WUM` writer - Wake up mode"]
pub type WUM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBN` reader - Data bit num"]
pub type DBN_R = crate::BitReader;
#[doc = "Field `DBN` writer - Data bit num"]
pub type DBN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UEN` reader - USART enable"]
pub type UEN_R = crate::BitReader;
#[doc = "Field `UEN` writer - USART enable"]
pub type UEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Send break frame"]
    #[inline(always)]
    pub fn sbf(&self) -> SBF_R {
        SBF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Receiver mute"]
    #[inline(always)]
    pub fn rm(&self) -> RM_R {
        RM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receiver enable"]
    #[inline(always)]
    pub fn ren(&self) -> REN_R {
        REN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmitter enable"]
    #[inline(always)]
    pub fn ten(&self) -> TEN_R {
        TEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IDLE interrupt enable"]
    #[inline(always)]
    pub fn idleien(&self) -> IDLEIEN_R {
        IDLEIEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RDBF interrupt enable"]
    #[inline(always)]
    pub fn rdbfien(&self) -> RDBFIEN_R {
        RDBFIEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TDC interrupt enable"]
    #[inline(always)]
    pub fn tdcien(&self) -> TDCIEN_R {
        TDCIEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TDBE interrupt enable"]
    #[inline(always)]
    pub fn tdbeien(&self) -> TDBEIEN_R {
        TDBEIEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - PERR interrupt enable"]
    #[inline(always)]
    pub fn perrien(&self) -> PERRIEN_R {
        PERRIEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Parity selection"]
    #[inline(always)]
    pub fn psel(&self) -> PSEL_R {
        PSEL_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Parity enable"]
    #[inline(always)]
    pub fn pen(&self) -> PEN_R {
        PEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Wake up mode"]
    #[inline(always)]
    pub fn wum(&self) -> WUM_R {
        WUM_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Data bit num"]
    #[inline(always)]
    pub fn dbn(&self) -> DBN_R {
        DBN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - USART enable"]
    #[inline(always)]
    pub fn uen(&self) -> UEN_R {
        UEN_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL1")
            .field("uen", &format_args!("{}", self.uen().bit()))
            .field("dbn", &format_args!("{}", self.dbn().bit()))
            .field("wum", &format_args!("{}", self.wum().bit()))
            .field("pen", &format_args!("{}", self.pen().bit()))
            .field("psel", &format_args!("{}", self.psel().bit()))
            .field("perrien", &format_args!("{}", self.perrien().bit()))
            .field("tdbeien", &format_args!("{}", self.tdbeien().bit()))
            .field("tdcien", &format_args!("{}", self.tdcien().bit()))
            .field("rdbfien", &format_args!("{}", self.rdbfien().bit()))
            .field("idleien", &format_args!("{}", self.idleien().bit()))
            .field("ten", &format_args!("{}", self.ten().bit()))
            .field("ren", &format_args!("{}", self.ren().bit()))
            .field("rm", &format_args!("{}", self.rm().bit()))
            .field("sbf", &format_args!("{}", self.sbf().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<CTRL1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - Send break frame"]
    #[inline(always)]
    #[must_use]
    pub fn sbf(&mut self) -> SBF_W<CTRL1_SPEC> {
        SBF_W::new(self, 0)
    }
    #[doc = "Bit 1 - Receiver mute"]
    #[inline(always)]
    #[must_use]
    pub fn rm(&mut self) -> RM_W<CTRL1_SPEC> {
        RM_W::new(self, 1)
    }
    #[doc = "Bit 2 - Receiver enable"]
    #[inline(always)]
    #[must_use]
    pub fn ren(&mut self) -> REN_W<CTRL1_SPEC> {
        REN_W::new(self, 2)
    }
    #[doc = "Bit 3 - Transmitter enable"]
    #[inline(always)]
    #[must_use]
    pub fn ten(&mut self) -> TEN_W<CTRL1_SPEC> {
        TEN_W::new(self, 3)
    }
    #[doc = "Bit 4 - IDLE interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn idleien(&mut self) -> IDLEIEN_W<CTRL1_SPEC> {
        IDLEIEN_W::new(self, 4)
    }
    #[doc = "Bit 5 - RDBF interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rdbfien(&mut self) -> RDBFIEN_W<CTRL1_SPEC> {
        RDBFIEN_W::new(self, 5)
    }
    #[doc = "Bit 6 - TDC interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tdcien(&mut self) -> TDCIEN_W<CTRL1_SPEC> {
        TDCIEN_W::new(self, 6)
    }
    #[doc = "Bit 7 - TDBE interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tdbeien(&mut self) -> TDBEIEN_W<CTRL1_SPEC> {
        TDBEIEN_W::new(self, 7)
    }
    #[doc = "Bit 8 - PERR interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn perrien(&mut self) -> PERRIEN_W<CTRL1_SPEC> {
        PERRIEN_W::new(self, 8)
    }
    #[doc = "Bit 9 - Parity selection"]
    #[inline(always)]
    #[must_use]
    pub fn psel(&mut self) -> PSEL_W<CTRL1_SPEC> {
        PSEL_W::new(self, 9)
    }
    #[doc = "Bit 10 - Parity enable"]
    #[inline(always)]
    #[must_use]
    pub fn pen(&mut self) -> PEN_W<CTRL1_SPEC> {
        PEN_W::new(self, 10)
    }
    #[doc = "Bit 11 - Wake up mode"]
    #[inline(always)]
    #[must_use]
    pub fn wum(&mut self) -> WUM_W<CTRL1_SPEC> {
        WUM_W::new(self, 11)
    }
    #[doc = "Bit 12 - Data bit num"]
    #[inline(always)]
    #[must_use]
    pub fn dbn(&mut self) -> DBN_W<CTRL1_SPEC> {
        DBN_W::new(self, 12)
    }
    #[doc = "Bit 13 - USART enable"]
    #[inline(always)]
    #[must_use]
    pub fn uen(&mut self) -> UEN_W<CTRL1_SPEC> {
        UEN_W::new(self, 13)
    }
}
#[doc = "Control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL1_SPEC;
impl crate::RegisterSpec for CTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl1::R`](R) reader structure"]
impl crate::Readable for CTRL1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl1::W`](W) writer structure"]
impl crate::Writable for CTRL1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL1 to value 0"]
impl crate::Resettable for CTRL1_SPEC {
    const RESET_VALUE: u32 = 0;
}
