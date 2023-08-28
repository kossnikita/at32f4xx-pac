#[doc = "Register `CMD_W1` reader"]
pub type R = crate::R<CMD_W1_SPEC>;
#[doc = "Register `CMD_W1` writer"]
pub type W = crate::W<CMD_W1_SPEC>;
#[doc = "Field `ADRLEN` reader - SPI address length"]
pub type ADRLEN_R = crate::FieldReader;
#[doc = "Field `ADRLEN` writer - SPI address length"]
pub type ADRLEN_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `DUM2` reader - Second dummy state cycle"]
pub type DUM2_R = crate::FieldReader;
#[doc = "Field `DUM2` writer - Second dummy state cycle"]
pub type DUM2_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `INSLEN` reader - Instruction code length"]
pub type INSLEN_R = crate::FieldReader;
#[doc = "Field `INSLEN` writer - Instruction code length"]
pub type INSLEN_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `PEMEN` reader - Perfrmance enhance mode enable"]
pub type PEMEN_R = crate::BitReader;
#[doc = "Field `PEMEN` writer - Perfrmance enhance mode enable"]
pub type PEMEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:2 - SPI address length"]
    #[inline(always)]
    pub fn adrlen(&self) -> ADRLEN_R {
        ADRLEN_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 16:23 - Second dummy state cycle"]
    #[inline(always)]
    pub fn dum2(&self) -> DUM2_R {
        DUM2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:25 - Instruction code length"]
    #[inline(always)]
    pub fn inslen(&self) -> INSLEN_R {
        INSLEN_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 28 - Perfrmance enhance mode enable"]
    #[inline(always)]
    pub fn pemen(&self) -> PEMEN_R {
        PEMEN_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - SPI address length"]
    #[inline(always)]
    #[must_use]
    pub fn adrlen(&mut self) -> ADRLEN_W<CMD_W1_SPEC, 0> {
        ADRLEN_W::new(self)
    }
    #[doc = "Bits 16:23 - Second dummy state cycle"]
    #[inline(always)]
    #[must_use]
    pub fn dum2(&mut self) -> DUM2_W<CMD_W1_SPEC, 16> {
        DUM2_W::new(self)
    }
    #[doc = "Bits 24:25 - Instruction code length"]
    #[inline(always)]
    #[must_use]
    pub fn inslen(&mut self) -> INSLEN_W<CMD_W1_SPEC, 24> {
        INSLEN_W::new(self)
    }
    #[doc = "Bit 28 - Perfrmance enhance mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn pemen(&mut self) -> PEMEN_W<CMD_W1_SPEC, 28> {
        PEMEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Command word 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd_w1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd_w1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMD_W1_SPEC;
impl crate::RegisterSpec for CMD_W1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmd_w1::R`](R) reader structure"]
impl crate::Readable for CMD_W1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cmd_w1::W`](W) writer structure"]
impl crate::Writable for CMD_W1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMD_W1 to value 0x0100_0003"]
impl crate::Resettable for CMD_W1_SPEC {
    const RESET_VALUE: Self::Ux = 0x0100_0003;
}