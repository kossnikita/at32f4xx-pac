#[doc = "Register `CTRL1` reader"]
pub type R = crate::R<CTRL1_SPEC>;
#[doc = "Register `CTRL1` writer"]
pub type W = crate::W<CTRL1_SPEC>;
#[doc = "Field `CLKPHA` reader - Clock phase"]
pub type CLKPHA_R = crate::BitReader;
#[doc = "Field `CLKPHA` writer - Clock phase"]
pub type CLKPHA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKPOL` reader - Clock polarity"]
pub type CLKPOL_R = crate::BitReader;
#[doc = "Field `CLKPOL` writer - Clock polarity"]
pub type CLKPOL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSTEN` reader - Master enable"]
pub type MSTEN_R = crate::BitReader;
#[doc = "Field `MSTEN` writer - Master enable"]
pub type MSTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MDIV2_0` reader - Master clock frequency division bit2-0"]
pub type MDIV2_0_R = crate::FieldReader;
#[doc = "Field `MDIV2_0` writer - Master clock frequency division bit2-0"]
pub type MDIV2_0_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SPIEN` reader - SPI enable"]
pub type SPIEN_R = crate::BitReader;
#[doc = "Field `SPIEN` writer - SPI enable"]
pub type SPIEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LTF` reader - LSB transmit first"]
pub type LTF_R = crate::BitReader;
#[doc = "Field `LTF` writer - LSB transmit first"]
pub type LTF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWCSIL` reader - Software CS internal level"]
pub type SWCSIL_R = crate::BitReader;
#[doc = "Field `SWCSIL` writer - Software CS internal level"]
pub type SWCSIL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWCSEN` reader - Software CS enable"]
pub type SWCSEN_R = crate::BitReader;
#[doc = "Field `SWCSEN` writer - Software CS enable"]
pub type SWCSEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ORA` reader - Only receive active"]
pub type ORA_R = crate::BitReader;
#[doc = "Field `ORA` writer - Only receive active"]
pub type ORA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FBN` reader - frame bit num"]
pub type FBN_R = crate::BitReader;
#[doc = "Field `FBN` writer - frame bit num"]
pub type FBN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NTC` reader - Next transmission CRC"]
pub type NTC_R = crate::BitReader;
#[doc = "Field `NTC` writer - Next transmission CRC"]
pub type NTC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCEN` reader - CRC calculation enable"]
pub type CCEN_R = crate::BitReader;
#[doc = "Field `CCEN` writer - CRC calculation enable"]
pub type CCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLBTD` reader - Single line bidirectional half-duplex transmission direction"]
pub type SLBTD_R = crate::BitReader;
#[doc = "Field `SLBTD` writer - Single line bidirectional half-duplex transmission direction"]
pub type SLBTD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLBEN` reader - Single line bidirectional half-duplex enable"]
pub type SLBEN_R = crate::BitReader;
#[doc = "Field `SLBEN` writer - Single line bidirectional half-duplex enable"]
pub type SLBEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Clock phase"]
    #[inline(always)]
    pub fn clkpha(&self) -> CLKPHA_R {
        CLKPHA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clock polarity"]
    #[inline(always)]
    pub fn clkpol(&self) -> CLKPOL_R {
        CLKPOL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Master enable"]
    #[inline(always)]
    pub fn msten(&self) -> MSTEN_R {
        MSTEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:5 - Master clock frequency division bit2-0"]
    #[inline(always)]
    pub fn mdiv2_0(&self) -> MDIV2_0_R {
        MDIV2_0_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bit 6 - SPI enable"]
    #[inline(always)]
    pub fn spien(&self) -> SPIEN_R {
        SPIEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - LSB transmit first"]
    #[inline(always)]
    pub fn ltf(&self) -> LTF_R {
        LTF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Software CS internal level"]
    #[inline(always)]
    pub fn swcsil(&self) -> SWCSIL_R {
        SWCSIL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Software CS enable"]
    #[inline(always)]
    pub fn swcsen(&self) -> SWCSEN_R {
        SWCSEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Only receive active"]
    #[inline(always)]
    pub fn ora(&self) -> ORA_R {
        ORA_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - frame bit num"]
    #[inline(always)]
    pub fn fbn(&self) -> FBN_R {
        FBN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Next transmission CRC"]
    #[inline(always)]
    pub fn ntc(&self) -> NTC_R {
        NTC_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - CRC calculation enable"]
    #[inline(always)]
    pub fn ccen(&self) -> CCEN_R {
        CCEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Single line bidirectional half-duplex transmission direction"]
    #[inline(always)]
    pub fn slbtd(&self) -> SLBTD_R {
        SLBTD_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Single line bidirectional half-duplex enable"]
    #[inline(always)]
    pub fn slben(&self) -> SLBEN_R {
        SLBEN_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL1")
            .field("slben", &self.slben())
            .field("slbtd", &self.slbtd())
            .field("ccen", &self.ccen())
            .field("ntc", &self.ntc())
            .field("fbn", &self.fbn())
            .field("ora", &self.ora())
            .field("swcsen", &self.swcsen())
            .field("swcsil", &self.swcsil())
            .field("ltf", &self.ltf())
            .field("spien", &self.spien())
            .field("mdiv2_0", &self.mdiv2_0())
            .field("msten", &self.msten())
            .field("clkpol", &self.clkpol())
            .field("clkpha", &self.clkpha())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Clock phase"]
    #[inline(always)]
    #[must_use]
    pub fn clkpha(&mut self) -> CLKPHA_W<CTRL1_SPEC> {
        CLKPHA_W::new(self, 0)
    }
    #[doc = "Bit 1 - Clock polarity"]
    #[inline(always)]
    #[must_use]
    pub fn clkpol(&mut self) -> CLKPOL_W<CTRL1_SPEC> {
        CLKPOL_W::new(self, 1)
    }
    #[doc = "Bit 2 - Master enable"]
    #[inline(always)]
    #[must_use]
    pub fn msten(&mut self) -> MSTEN_W<CTRL1_SPEC> {
        MSTEN_W::new(self, 2)
    }
    #[doc = "Bits 3:5 - Master clock frequency division bit2-0"]
    #[inline(always)]
    #[must_use]
    pub fn mdiv2_0(&mut self) -> MDIV2_0_W<CTRL1_SPEC> {
        MDIV2_0_W::new(self, 3)
    }
    #[doc = "Bit 6 - SPI enable"]
    #[inline(always)]
    #[must_use]
    pub fn spien(&mut self) -> SPIEN_W<CTRL1_SPEC> {
        SPIEN_W::new(self, 6)
    }
    #[doc = "Bit 7 - LSB transmit first"]
    #[inline(always)]
    #[must_use]
    pub fn ltf(&mut self) -> LTF_W<CTRL1_SPEC> {
        LTF_W::new(self, 7)
    }
    #[doc = "Bit 8 - Software CS internal level"]
    #[inline(always)]
    #[must_use]
    pub fn swcsil(&mut self) -> SWCSIL_W<CTRL1_SPEC> {
        SWCSIL_W::new(self, 8)
    }
    #[doc = "Bit 9 - Software CS enable"]
    #[inline(always)]
    #[must_use]
    pub fn swcsen(&mut self) -> SWCSEN_W<CTRL1_SPEC> {
        SWCSEN_W::new(self, 9)
    }
    #[doc = "Bit 10 - Only receive active"]
    #[inline(always)]
    #[must_use]
    pub fn ora(&mut self) -> ORA_W<CTRL1_SPEC> {
        ORA_W::new(self, 10)
    }
    #[doc = "Bit 11 - frame bit num"]
    #[inline(always)]
    #[must_use]
    pub fn fbn(&mut self) -> FBN_W<CTRL1_SPEC> {
        FBN_W::new(self, 11)
    }
    #[doc = "Bit 12 - Next transmission CRC"]
    #[inline(always)]
    #[must_use]
    pub fn ntc(&mut self) -> NTC_W<CTRL1_SPEC> {
        NTC_W::new(self, 12)
    }
    #[doc = "Bit 13 - CRC calculation enable"]
    #[inline(always)]
    #[must_use]
    pub fn ccen(&mut self) -> CCEN_W<CTRL1_SPEC> {
        CCEN_W::new(self, 13)
    }
    #[doc = "Bit 14 - Single line bidirectional half-duplex transmission direction"]
    #[inline(always)]
    #[must_use]
    pub fn slbtd(&mut self) -> SLBTD_W<CTRL1_SPEC> {
        SLBTD_W::new(self, 14)
    }
    #[doc = "Bit 15 - Single line bidirectional half-duplex enable"]
    #[inline(always)]
    #[must_use]
    pub fn slben(&mut self) -> SLBEN_W<CTRL1_SPEC> {
        SLBEN_W::new(self, 15)
    }
}
#[doc = "control register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
