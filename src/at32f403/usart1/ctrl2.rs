#[doc = "Register `CTRL2` reader"]
pub type R = crate::R<CTRL2_SPEC>;
#[doc = "Register `CTRL2` writer"]
pub type W = crate::W<CTRL2_SPEC>;
#[doc = "Field `ID` reader - USART identification"]
pub type ID_R = crate::FieldReader;
#[doc = "Field `ID` writer - USART identification"]
pub type ID_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `BFBN` reader - Break frame bit num"]
pub type BFBN_R = crate::BitReader;
#[doc = "Field `BFBN` writer - Break frame bit num"]
pub type BFBN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BFIEN` reader - Break frame interrupt enable"]
pub type BFIEN_R = crate::BitReader;
#[doc = "Field `BFIEN` writer - Break frame interrupt enable"]
pub type BFIEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LBCP` reader - Last bit clock pulse"]
pub type LBCP_R = crate::BitReader;
#[doc = "Field `LBCP` writer - Last bit clock pulse"]
pub type LBCP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKPHA` reader - Clock phase"]
pub type CLKPHA_R = crate::BitReader;
#[doc = "Field `CLKPHA` writer - Clock phase"]
pub type CLKPHA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKPOL` reader - Clock polarity"]
pub type CLKPOL_R = crate::BitReader;
#[doc = "Field `CLKPOL` writer - Clock polarity"]
pub type CLKPOL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKEN` reader - Clock enable"]
pub type CLKEN_R = crate::BitReader;
#[doc = "Field `CLKEN` writer - Clock enable"]
pub type CLKEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STOPBN` reader - STOP bit num"]
pub type STOPBN_R = crate::FieldReader;
#[doc = "Field `STOPBN` writer - STOP bit num"]
pub type STOPBN_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LINEN` reader - LIN mode enable"]
pub type LINEN_R = crate::BitReader;
#[doc = "Field `LINEN` writer - LIN mode enable"]
pub type LINEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - USART identification"]
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 5 - Break frame bit num"]
    #[inline(always)]
    pub fn bfbn(&self) -> BFBN_R {
        BFBN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Break frame interrupt enable"]
    #[inline(always)]
    pub fn bfien(&self) -> BFIEN_R {
        BFIEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Last bit clock pulse"]
    #[inline(always)]
    pub fn lbcp(&self) -> LBCP_R {
        LBCP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Clock phase"]
    #[inline(always)]
    pub fn clkpha(&self) -> CLKPHA_R {
        CLKPHA_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Clock polarity"]
    #[inline(always)]
    pub fn clkpol(&self) -> CLKPOL_R {
        CLKPOL_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Clock enable"]
    #[inline(always)]
    pub fn clken(&self) -> CLKEN_R {
        CLKEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - STOP bit num"]
    #[inline(always)]
    pub fn stopbn(&self) -> STOPBN_R {
        STOPBN_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - LIN mode enable"]
    #[inline(always)]
    pub fn linen(&self) -> LINEN_R {
        LINEN_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL2")
            .field("linen", &self.linen())
            .field("stopbn", &self.stopbn())
            .field("clken", &self.clken())
            .field("clkpol", &self.clkpol())
            .field("clkpha", &self.clkpha())
            .field("lbcp", &self.lbcp())
            .field("bfien", &self.bfien())
            .field("bfbn", &self.bfbn())
            .field("id", &self.id())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - USART identification"]
    #[inline(always)]
    #[must_use]
    pub fn id(&mut self) -> ID_W<CTRL2_SPEC> {
        ID_W::new(self, 0)
    }
    #[doc = "Bit 5 - Break frame bit num"]
    #[inline(always)]
    #[must_use]
    pub fn bfbn(&mut self) -> BFBN_W<CTRL2_SPEC> {
        BFBN_W::new(self, 5)
    }
    #[doc = "Bit 6 - Break frame interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn bfien(&mut self) -> BFIEN_W<CTRL2_SPEC> {
        BFIEN_W::new(self, 6)
    }
    #[doc = "Bit 8 - Last bit clock pulse"]
    #[inline(always)]
    #[must_use]
    pub fn lbcp(&mut self) -> LBCP_W<CTRL2_SPEC> {
        LBCP_W::new(self, 8)
    }
    #[doc = "Bit 9 - Clock phase"]
    #[inline(always)]
    #[must_use]
    pub fn clkpha(&mut self) -> CLKPHA_W<CTRL2_SPEC> {
        CLKPHA_W::new(self, 9)
    }
    #[doc = "Bit 10 - Clock polarity"]
    #[inline(always)]
    #[must_use]
    pub fn clkpol(&mut self) -> CLKPOL_W<CTRL2_SPEC> {
        CLKPOL_W::new(self, 10)
    }
    #[doc = "Bit 11 - Clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn clken(&mut self) -> CLKEN_W<CTRL2_SPEC> {
        CLKEN_W::new(self, 11)
    }
    #[doc = "Bits 12:13 - STOP bit num"]
    #[inline(always)]
    #[must_use]
    pub fn stopbn(&mut self) -> STOPBN_W<CTRL2_SPEC> {
        STOPBN_W::new(self, 12)
    }
    #[doc = "Bit 14 - LIN mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn linen(&mut self) -> LINEN_W<CTRL2_SPEC> {
        LINEN_W::new(self, 14)
    }
}
#[doc = "Control register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL2_SPEC;
impl crate::RegisterSpec for CTRL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl2::R`](R) reader structure"]
impl crate::Readable for CTRL2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl2::W`](W) writer structure"]
impl crate::Writable for CTRL2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL2 to value 0"]
impl crate::Resettable for CTRL2_SPEC {
    const RESET_VALUE: u32 = 0;
}
