#[doc = "Register `CTRL3` reader"]
pub type R = crate::R<CTRL3_SPEC>;
#[doc = "Register `CTRL3` writer"]
pub type W = crate::W<CTRL3_SPEC>;
#[doc = "Field `ERRIEN` reader - Error interrupt enable"]
pub type ERRIEN_R = crate::BitReader;
#[doc = "Field `ERRIEN` writer - Error interrupt enable"]
pub type ERRIEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IRDAEN` reader - IrDA enable"]
pub type IRDAEN_R = crate::BitReader;
#[doc = "Field `IRDAEN` writer - IrDA enable"]
pub type IRDAEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IRDALP` reader - IrDA low-power mode"]
pub type IRDALP_R = crate::BitReader;
#[doc = "Field `IRDALP` writer - IrDA low-power mode"]
pub type IRDALP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLBEN` reader - Single line bidirectional half-duplex enable"]
pub type SLBEN_R = crate::BitReader;
#[doc = "Field `SLBEN` writer - Single line bidirectional half-duplex enable"]
pub type SLBEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCNACKEN` reader - Smartcard NACK enable"]
pub type SCNACKEN_R = crate::BitReader;
#[doc = "Field `SCNACKEN` writer - Smartcard NACK enable"]
pub type SCNACKEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCMEN` reader - Smartcard mode enable"]
pub type SCMEN_R = crate::BitReader;
#[doc = "Field `SCMEN` writer - Smartcard mode enable"]
pub type SCMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAREN` reader - DMA receiver enable"]
pub type DMAREN_R = crate::BitReader;
#[doc = "Field `DMAREN` writer - DMA receiver enable"]
pub type DMAREN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMATEN` reader - DMA transmitter enable"]
pub type DMATEN_R = crate::BitReader;
#[doc = "Field `DMATEN` writer - DMA transmitter enable"]
pub type DMATEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTSEN` reader - RTS enable"]
pub type RTSEN_R = crate::BitReader;
#[doc = "Field `RTSEN` writer - RTS enable"]
pub type RTSEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTSEN` reader - CTS enable"]
pub type CTSEN_R = crate::BitReader;
#[doc = "Field `CTSEN` writer - CTS enable"]
pub type CTSEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTSCFIEN` reader - CTSCF interrupt enable"]
pub type CTSCFIEN_R = crate::BitReader;
#[doc = "Field `CTSCFIEN` writer - CTSCF interrupt enable"]
pub type CTSCFIEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Error interrupt enable"]
    #[inline(always)]
    pub fn errien(&self) -> ERRIEN_R {
        ERRIEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IrDA enable"]
    #[inline(always)]
    pub fn irdaen(&self) -> IRDAEN_R {
        IRDAEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - IrDA low-power mode"]
    #[inline(always)]
    pub fn irdalp(&self) -> IRDALP_R {
        IRDALP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Single line bidirectional half-duplex enable"]
    #[inline(always)]
    pub fn slben(&self) -> SLBEN_R {
        SLBEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Smartcard NACK enable"]
    #[inline(always)]
    pub fn scnacken(&self) -> SCNACKEN_R {
        SCNACKEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Smartcard mode enable"]
    #[inline(always)]
    pub fn scmen(&self) -> SCMEN_R {
        SCMEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - DMA receiver enable"]
    #[inline(always)]
    pub fn dmaren(&self) -> DMAREN_R {
        DMAREN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DMA transmitter enable"]
    #[inline(always)]
    pub fn dmaten(&self) -> DMATEN_R {
        DMATEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - RTS enable"]
    #[inline(always)]
    pub fn rtsen(&self) -> RTSEN_R {
        RTSEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CTS enable"]
    #[inline(always)]
    pub fn ctsen(&self) -> CTSEN_R {
        CTSEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - CTSCF interrupt enable"]
    #[inline(always)]
    pub fn ctscfien(&self) -> CTSCFIEN_R {
        CTSCFIEN_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL3")
            .field("ctscfien", &format_args!("{}", self.ctscfien().bit()))
            .field("ctsen", &format_args!("{}", self.ctsen().bit()))
            .field("rtsen", &format_args!("{}", self.rtsen().bit()))
            .field("dmaten", &format_args!("{}", self.dmaten().bit()))
            .field("dmaren", &format_args!("{}", self.dmaren().bit()))
            .field("scmen", &format_args!("{}", self.scmen().bit()))
            .field("scnacken", &format_args!("{}", self.scnacken().bit()))
            .field("slben", &format_args!("{}", self.slben().bit()))
            .field("irdalp", &format_args!("{}", self.irdalp().bit()))
            .field("irdaen", &format_args!("{}", self.irdaen().bit()))
            .field("errien", &format_args!("{}", self.errien().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<CTRL3_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn errien(&mut self) -> ERRIEN_W<CTRL3_SPEC> {
        ERRIEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - IrDA enable"]
    #[inline(always)]
    #[must_use]
    pub fn irdaen(&mut self) -> IRDAEN_W<CTRL3_SPEC> {
        IRDAEN_W::new(self, 1)
    }
    #[doc = "Bit 2 - IrDA low-power mode"]
    #[inline(always)]
    #[must_use]
    pub fn irdalp(&mut self) -> IRDALP_W<CTRL3_SPEC> {
        IRDALP_W::new(self, 2)
    }
    #[doc = "Bit 3 - Single line bidirectional half-duplex enable"]
    #[inline(always)]
    #[must_use]
    pub fn slben(&mut self) -> SLBEN_W<CTRL3_SPEC> {
        SLBEN_W::new(self, 3)
    }
    #[doc = "Bit 4 - Smartcard NACK enable"]
    #[inline(always)]
    #[must_use]
    pub fn scnacken(&mut self) -> SCNACKEN_W<CTRL3_SPEC> {
        SCNACKEN_W::new(self, 4)
    }
    #[doc = "Bit 5 - Smartcard mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn scmen(&mut self) -> SCMEN_W<CTRL3_SPEC> {
        SCMEN_W::new(self, 5)
    }
    #[doc = "Bit 6 - DMA receiver enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmaren(&mut self) -> DMAREN_W<CTRL3_SPEC> {
        DMAREN_W::new(self, 6)
    }
    #[doc = "Bit 7 - DMA transmitter enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmaten(&mut self) -> DMATEN_W<CTRL3_SPEC> {
        DMATEN_W::new(self, 7)
    }
    #[doc = "Bit 8 - RTS enable"]
    #[inline(always)]
    #[must_use]
    pub fn rtsen(&mut self) -> RTSEN_W<CTRL3_SPEC> {
        RTSEN_W::new(self, 8)
    }
    #[doc = "Bit 9 - CTS enable"]
    #[inline(always)]
    #[must_use]
    pub fn ctsen(&mut self) -> CTSEN_W<CTRL3_SPEC> {
        CTSEN_W::new(self, 9)
    }
    #[doc = "Bit 10 - CTSCF interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ctscfien(&mut self) -> CTSCFIEN_W<CTRL3_SPEC> {
        CTSCFIEN_W::new(self, 10)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Control register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL3_SPEC;
impl crate::RegisterSpec for CTRL3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl3::R`](R) reader structure"]
impl crate::Readable for CTRL3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl3::W`](W) writer structure"]
impl crate::Writable for CTRL3_SPEC {
    const ZEROS_BITMAP: Self::Ux = 0;
    const ONES_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL3 to value 0"]
impl crate::Resettable for CTRL3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
