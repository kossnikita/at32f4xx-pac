#[doc = "Register `CTRL1` reader"]
pub type R = crate::R<CTRL1_SPEC>;
#[doc = "Register `CTRL1` writer"]
pub type W = crate::W<CTRL1_SPEC>;
#[doc = "Field `CLKPHA` reader - Clock phase"]
pub type CLKPHA_R = crate::BitReader;
#[doc = "Field `CLKPHA` writer - Clock phase"]
pub type CLKPHA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CLKPOL` reader - Clock polarity"]
pub type CLKPOL_R = crate::BitReader;
#[doc = "Field `CLKPOL` writer - Clock polarity"]
pub type CLKPOL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MSTEN` reader - Master enable"]
pub type MSTEN_R = crate::BitReader;
#[doc = "Field `MSTEN` writer - Master enable"]
pub type MSTEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MDIV2_0` reader - Master clock frequency division bit2-0"]
pub type MDIV2_0_R = crate::FieldReader;
#[doc = "Field `MDIV2_0` writer - Master clock frequency division bit2-0"]
pub type MDIV2_0_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `SPIEN` reader - SPI enable"]
pub type SPIEN_R = crate::BitReader;
#[doc = "Field `SPIEN` writer - SPI enable"]
pub type SPIEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LTF` reader - LSB transmit first"]
pub type LTF_R = crate::BitReader;
#[doc = "Field `LTF` writer - LSB transmit first"]
pub type LTF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SWCSIL` reader - Software CS internal level"]
pub type SWCSIL_R = crate::BitReader;
#[doc = "Field `SWCSIL` writer - Software CS internal level"]
pub type SWCSIL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SWCSEN` reader - Software CS enable"]
pub type SWCSEN_R = crate::BitReader;
#[doc = "Field `SWCSEN` writer - Software CS enable"]
pub type SWCSEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ORA` reader - Only receive active"]
pub type ORA_R = crate::BitReader;
#[doc = "Field `ORA` writer - Only receive active"]
pub type ORA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FBN` reader - frame bit num"]
pub type FBN_R = crate::BitReader;
#[doc = "Field `FBN` writer - frame bit num"]
pub type FBN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `NTC` reader - Next transmission CRC"]
pub type NTC_R = crate::BitReader;
#[doc = "Field `NTC` writer - Next transmission CRC"]
pub type NTC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CCEN` reader - CRC calculation enable"]
pub type CCEN_R = crate::BitReader;
#[doc = "Field `CCEN` writer - CRC calculation enable"]
pub type CCEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SLBTD` reader - Single line bidirectional half-duplex transmission direction"]
pub type SLBTD_R = crate::BitReader;
#[doc = "Field `SLBTD` writer - Single line bidirectional half-duplex transmission direction"]
pub type SLBTD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SLBEN` reader - Single line bidirectional half-duplex enable"]
pub type SLBEN_R = crate::BitReader;
#[doc = "Field `SLBEN` writer - Single line bidirectional half-duplex enable"]
pub type SLBEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
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
            .field("slben", &format_args!("{}", self.slben().bit()))
            .field("slbtd", &format_args!("{}", self.slbtd().bit()))
            .field("ccen", &format_args!("{}", self.ccen().bit()))
            .field("ntc", &format_args!("{}", self.ntc().bit()))
            .field("fbn", &format_args!("{}", self.fbn().bit()))
            .field("ora", &format_args!("{}", self.ora().bit()))
            .field("swcsen", &format_args!("{}", self.swcsen().bit()))
            .field("swcsil", &format_args!("{}", self.swcsil().bit()))
            .field("ltf", &format_args!("{}", self.ltf().bit()))
            .field("spien", &format_args!("{}", self.spien().bit()))
            .field("mdiv2_0", &format_args!("{}", self.mdiv2_0().bits()))
            .field("msten", &format_args!("{}", self.msten().bit()))
            .field("clkpol", &format_args!("{}", self.clkpol().bit()))
            .field("clkpha", &format_args!("{}", self.clkpha().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<CTRL1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Clock phase"]
    #[inline(always)]
    #[must_use]
    pub fn clkpha(&mut self) -> CLKPHA_W<CTRL1_SPEC, 0> {
        CLKPHA_W::new(self)
    }
    #[doc = "Bit 1 - Clock polarity"]
    #[inline(always)]
    #[must_use]
    pub fn clkpol(&mut self) -> CLKPOL_W<CTRL1_SPEC, 1> {
        CLKPOL_W::new(self)
    }
    #[doc = "Bit 2 - Master enable"]
    #[inline(always)]
    #[must_use]
    pub fn msten(&mut self) -> MSTEN_W<CTRL1_SPEC, 2> {
        MSTEN_W::new(self)
    }
    #[doc = "Bits 3:5 - Master clock frequency division bit2-0"]
    #[inline(always)]
    #[must_use]
    pub fn mdiv2_0(&mut self) -> MDIV2_0_W<CTRL1_SPEC, 3> {
        MDIV2_0_W::new(self)
    }
    #[doc = "Bit 6 - SPI enable"]
    #[inline(always)]
    #[must_use]
    pub fn spien(&mut self) -> SPIEN_W<CTRL1_SPEC, 6> {
        SPIEN_W::new(self)
    }
    #[doc = "Bit 7 - LSB transmit first"]
    #[inline(always)]
    #[must_use]
    pub fn ltf(&mut self) -> LTF_W<CTRL1_SPEC, 7> {
        LTF_W::new(self)
    }
    #[doc = "Bit 8 - Software CS internal level"]
    #[inline(always)]
    #[must_use]
    pub fn swcsil(&mut self) -> SWCSIL_W<CTRL1_SPEC, 8> {
        SWCSIL_W::new(self)
    }
    #[doc = "Bit 9 - Software CS enable"]
    #[inline(always)]
    #[must_use]
    pub fn swcsen(&mut self) -> SWCSEN_W<CTRL1_SPEC, 9> {
        SWCSEN_W::new(self)
    }
    #[doc = "Bit 10 - Only receive active"]
    #[inline(always)]
    #[must_use]
    pub fn ora(&mut self) -> ORA_W<CTRL1_SPEC, 10> {
        ORA_W::new(self)
    }
    #[doc = "Bit 11 - frame bit num"]
    #[inline(always)]
    #[must_use]
    pub fn fbn(&mut self) -> FBN_W<CTRL1_SPEC, 11> {
        FBN_W::new(self)
    }
    #[doc = "Bit 12 - Next transmission CRC"]
    #[inline(always)]
    #[must_use]
    pub fn ntc(&mut self) -> NTC_W<CTRL1_SPEC, 12> {
        NTC_W::new(self)
    }
    #[doc = "Bit 13 - CRC calculation enable"]
    #[inline(always)]
    #[must_use]
    pub fn ccen(&mut self) -> CCEN_W<CTRL1_SPEC, 13> {
        CCEN_W::new(self)
    }
    #[doc = "Bit 14 - Single line bidirectional half-duplex transmission direction"]
    #[inline(always)]
    #[must_use]
    pub fn slbtd(&mut self) -> SLBTD_W<CTRL1_SPEC, 14> {
        SLBTD_W::new(self)
    }
    #[doc = "Bit 15 - Single line bidirectional half-duplex enable"]
    #[inline(always)]
    #[must_use]
    pub fn slben(&mut self) -> SLBEN_W<CTRL1_SPEC, 15> {
        SLBEN_W::new(self)
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
#[doc = "control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
