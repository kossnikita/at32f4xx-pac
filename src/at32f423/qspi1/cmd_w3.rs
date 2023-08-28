#[doc = "Register `CMD_W3` reader"]
pub type R = crate::R<CMD_W3_SPEC>;
#[doc = "Register `CMD_W3` writer"]
pub type W = crate::W<CMD_W3_SPEC>;
#[doc = "Field `WEN` reader - Write data enable"]
pub type WEN_R = crate::BitReader;
#[doc = "Field `WEN` writer - Write data enable"]
pub type WEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RSTSEN` reader - Read spi status enable"]
pub type RSTSEN_R = crate::BitReader;
#[doc = "Field `RSTSEN` writer - Read spi status enable"]
pub type RSTSEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RSTSC` reader - Read spi status configure"]
pub type RSTSC_R = crate::BitReader;
#[doc = "Field `RSTSC` writer - Read spi status configure"]
pub type RSTSC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OPMODE` reader - SPI operate mode"]
pub type OPMODE_R = crate::FieldReader;
#[doc = "Field `OPMODE` writer - SPI operate mode"]
pub type OPMODE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `PEMOPC` reader - Performance enhance mode operate code"]
pub type PEMOPC_R = crate::FieldReader;
#[doc = "Field `PEMOPC` writer - Performance enhance mode operate code"]
pub type PEMOPC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `INSC` reader - Instruction code"]
pub type INSC_R = crate::FieldReader;
#[doc = "Field `INSC` writer - Instruction code"]
pub type INSC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bit 1 - Write data enable"]
    #[inline(always)]
    pub fn wen(&self) -> WEN_R {
        WEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Read spi status enable"]
    #[inline(always)]
    pub fn rstsen(&self) -> RSTSEN_R {
        RSTSEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Read spi status configure"]
    #[inline(always)]
    pub fn rstsc(&self) -> RSTSC_R {
        RSTSC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 5:7 - SPI operate mode"]
    #[inline(always)]
    pub fn opmode(&self) -> OPMODE_R {
        OPMODE_R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 16:23 - Performance enhance mode operate code"]
    #[inline(always)]
    pub fn pemopc(&self) -> PEMOPC_R {
        PEMOPC_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Instruction code"]
    #[inline(always)]
    pub fn insc(&self) -> INSC_R {
        INSC_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Write data enable"]
    #[inline(always)]
    #[must_use]
    pub fn wen(&mut self) -> WEN_W<CMD_W3_SPEC, 1> {
        WEN_W::new(self)
    }
    #[doc = "Bit 2 - Read spi status enable"]
    #[inline(always)]
    #[must_use]
    pub fn rstsen(&mut self) -> RSTSEN_W<CMD_W3_SPEC, 2> {
        RSTSEN_W::new(self)
    }
    #[doc = "Bit 3 - Read spi status configure"]
    #[inline(always)]
    #[must_use]
    pub fn rstsc(&mut self) -> RSTSC_W<CMD_W3_SPEC, 3> {
        RSTSC_W::new(self)
    }
    #[doc = "Bits 5:7 - SPI operate mode"]
    #[inline(always)]
    #[must_use]
    pub fn opmode(&mut self) -> OPMODE_W<CMD_W3_SPEC, 5> {
        OPMODE_W::new(self)
    }
    #[doc = "Bits 16:23 - Performance enhance mode operate code"]
    #[inline(always)]
    #[must_use]
    pub fn pemopc(&mut self) -> PEMOPC_W<CMD_W3_SPEC, 16> {
        PEMOPC_W::new(self)
    }
    #[doc = "Bits 24:31 - Instruction code"]
    #[inline(always)]
    #[must_use]
    pub fn insc(&mut self) -> INSC_W<CMD_W3_SPEC, 24> {
        INSC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Command word 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd_w3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd_w3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMD_W3_SPEC;
impl crate::RegisterSpec for CMD_W3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmd_w3::R`](R) reader structure"]
impl crate::Readable for CMD_W3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cmd_w3::W`](W) writer structure"]
impl crate::Writable for CMD_W3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMD_W3 to value 0"]
impl crate::Resettable for CMD_W3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
