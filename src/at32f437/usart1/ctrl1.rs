#[doc = "Register `CTRL1` reader"]
pub type R = crate::R<CTRL1_SPEC>;
#[doc = "Register `CTRL1` writer"]
pub type W = crate::W<CTRL1_SPEC>;
#[doc = "Field `SBF` reader - Send break frame"]
pub type SBF_R = crate::BitReader;
#[doc = "Field `SBF` writer - Send break frame"]
pub type SBF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RM` reader - Receiver mute"]
pub type RM_R = crate::BitReader;
#[doc = "Field `RM` writer - Receiver mute"]
pub type RM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `REN` reader - Receiver enable"]
pub type REN_R = crate::BitReader;
#[doc = "Field `REN` writer - Receiver enable"]
pub type REN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TEN` reader - Transmitter enable"]
pub type TEN_R = crate::BitReader;
#[doc = "Field `TEN` writer - Transmitter enable"]
pub type TEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `IDLEIEN` reader - IDLE interrupt enable"]
pub type IDLEIEN_R = crate::BitReader;
#[doc = "Field `IDLEIEN` writer - IDLE interrupt enable"]
pub type IDLEIEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RDBFIEN` reader - RDBF interrupt enable"]
pub type RDBFIEN_R = crate::BitReader;
#[doc = "Field `RDBFIEN` writer - RDBF interrupt enable"]
pub type RDBFIEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TDCIEN` reader - TDC interrupt enable"]
pub type TDCIEN_R = crate::BitReader;
#[doc = "Field `TDCIEN` writer - TDC interrupt enable"]
pub type TDCIEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TDBEIEN` reader - TDBE interrupt enable"]
pub type TDBEIEN_R = crate::BitReader;
#[doc = "Field `TDBEIEN` writer - TDBE interrupt enable"]
pub type TDBEIEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PERRIEN` reader - PERR interrupt enable"]
pub type PERRIEN_R = crate::BitReader;
#[doc = "Field `PERRIEN` writer - PERR interrupt enable"]
pub type PERRIEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PSEL` reader - Parity selection"]
pub type PSEL_R = crate::BitReader;
#[doc = "Field `PSEL` writer - Parity selection"]
pub type PSEL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PEN` reader - Parity enable"]
pub type PEN_R = crate::BitReader;
#[doc = "Field `PEN` writer - Parity enable"]
pub type PEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WUM` reader - Wake up mode"]
pub type WUM_R = crate::BitReader;
#[doc = "Field `WUM` writer - Wake up mode"]
pub type WUM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DBN0` reader - low bit for Data bit num"]
pub type DBN0_R = crate::BitReader;
#[doc = "Field `DBN0` writer - low bit for Data bit num"]
pub type DBN0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `UEN` reader - USART enable"]
pub type UEN_R = crate::BitReader;
#[doc = "Field `UEN` writer - USART enable"]
pub type UEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TCDT` reader - transmit complete delay time"]
pub type TCDT_R = crate::FieldReader;
#[doc = "Field `TCDT` writer - transmit complete delay time"]
pub type TCDT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `TSDT` reader - transmit start delay time"]
pub type TSDT_R = crate::FieldReader;
#[doc = "Field `TSDT` writer - transmit start delay time"]
pub type TSDT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `DBN1` reader - high bit for Data bit num"]
pub type DBN1_R = crate::BitReader;
#[doc = "Field `DBN1` writer - high bit for Data bit num"]
pub type DBN1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
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
    #[doc = "Bit 12 - low bit for Data bit num"]
    #[inline(always)]
    pub fn dbn0(&self) -> DBN0_R {
        DBN0_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - USART enable"]
    #[inline(always)]
    pub fn uen(&self) -> UEN_R {
        UEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 16:20 - transmit complete delay time"]
    #[inline(always)]
    pub fn tcdt(&self) -> TCDT_R {
        TCDT_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 21:25 - transmit start delay time"]
    #[inline(always)]
    pub fn tsdt(&self) -> TSDT_R {
        TSDT_R::new(((self.bits >> 21) & 0x1f) as u8)
    }
    #[doc = "Bit 28 - high bit for Data bit num"]
    #[inline(always)]
    pub fn dbn1(&self) -> DBN1_R {
        DBN1_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Send break frame"]
    #[inline(always)]
    #[must_use]
    pub fn sbf(&mut self) -> SBF_W<CTRL1_SPEC, 0> {
        SBF_W::new(self)
    }
    #[doc = "Bit 1 - Receiver mute"]
    #[inline(always)]
    #[must_use]
    pub fn rm(&mut self) -> RM_W<CTRL1_SPEC, 1> {
        RM_W::new(self)
    }
    #[doc = "Bit 2 - Receiver enable"]
    #[inline(always)]
    #[must_use]
    pub fn ren(&mut self) -> REN_W<CTRL1_SPEC, 2> {
        REN_W::new(self)
    }
    #[doc = "Bit 3 - Transmitter enable"]
    #[inline(always)]
    #[must_use]
    pub fn ten(&mut self) -> TEN_W<CTRL1_SPEC, 3> {
        TEN_W::new(self)
    }
    #[doc = "Bit 4 - IDLE interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn idleien(&mut self) -> IDLEIEN_W<CTRL1_SPEC, 4> {
        IDLEIEN_W::new(self)
    }
    #[doc = "Bit 5 - RDBF interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rdbfien(&mut self) -> RDBFIEN_W<CTRL1_SPEC, 5> {
        RDBFIEN_W::new(self)
    }
    #[doc = "Bit 6 - TDC interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tdcien(&mut self) -> TDCIEN_W<CTRL1_SPEC, 6> {
        TDCIEN_W::new(self)
    }
    #[doc = "Bit 7 - TDBE interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tdbeien(&mut self) -> TDBEIEN_W<CTRL1_SPEC, 7> {
        TDBEIEN_W::new(self)
    }
    #[doc = "Bit 8 - PERR interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn perrien(&mut self) -> PERRIEN_W<CTRL1_SPEC, 8> {
        PERRIEN_W::new(self)
    }
    #[doc = "Bit 9 - Parity selection"]
    #[inline(always)]
    #[must_use]
    pub fn psel(&mut self) -> PSEL_W<CTRL1_SPEC, 9> {
        PSEL_W::new(self)
    }
    #[doc = "Bit 10 - Parity enable"]
    #[inline(always)]
    #[must_use]
    pub fn pen(&mut self) -> PEN_W<CTRL1_SPEC, 10> {
        PEN_W::new(self)
    }
    #[doc = "Bit 11 - Wake up mode"]
    #[inline(always)]
    #[must_use]
    pub fn wum(&mut self) -> WUM_W<CTRL1_SPEC, 11> {
        WUM_W::new(self)
    }
    #[doc = "Bit 12 - low bit for Data bit num"]
    #[inline(always)]
    #[must_use]
    pub fn dbn0(&mut self) -> DBN0_W<CTRL1_SPEC, 12> {
        DBN0_W::new(self)
    }
    #[doc = "Bit 13 - USART enable"]
    #[inline(always)]
    #[must_use]
    pub fn uen(&mut self) -> UEN_W<CTRL1_SPEC, 13> {
        UEN_W::new(self)
    }
    #[doc = "Bits 16:20 - transmit complete delay time"]
    #[inline(always)]
    #[must_use]
    pub fn tcdt(&mut self) -> TCDT_W<CTRL1_SPEC, 16> {
        TCDT_W::new(self)
    }
    #[doc = "Bits 21:25 - transmit start delay time"]
    #[inline(always)]
    #[must_use]
    pub fn tsdt(&mut self) -> TSDT_W<CTRL1_SPEC, 21> {
        TSDT_W::new(self)
    }
    #[doc = "Bit 28 - high bit for Data bit num"]
    #[inline(always)]
    #[must_use]
    pub fn dbn1(&mut self) -> DBN1_W<CTRL1_SPEC, 28> {
        DBN1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL1 to value 0"]
impl crate::Resettable for CTRL1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
