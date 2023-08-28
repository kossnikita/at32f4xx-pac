#[doc = "Register `CFG` reader"]
pub type R = crate::R<CFG_SPEC>;
#[doc = "Register `CFG` writer"]
pub type W = crate::W<CFG_SPEC>;
#[doc = "Field `SCLKSEL` reader - System clock select"]
pub type SCLKSEL_R = crate::FieldReader;
#[doc = "Field `SCLKSEL` writer - System clock select"]
pub type SCLKSEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `SCLKSTS` reader - System Clock select Status"]
pub type SCLKSTS_R = crate::FieldReader;
#[doc = "Field `AHBDIV` reader - AHB division"]
pub type AHBDIV_R = crate::FieldReader;
#[doc = "Field `AHBDIV` writer - AHB division"]
pub type AHBDIV_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `APB1DIV` reader - APB1 division"]
pub type APB1DIV_R = crate::FieldReader;
#[doc = "Field `APB1DIV` writer - APB1 division"]
pub type APB1DIV_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `APB2DIV` reader - APB2 division"]
pub type APB2DIV_R = crate::FieldReader;
#[doc = "Field `APB2DIV` writer - APB2 division"]
pub type APB2DIV_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `ERTCDIV` reader - HEXT division for ERTC clock"]
pub type ERTCDIV_R = crate::FieldReader;
#[doc = "Field `ERTCDIV` writer - HEXT division for ERTC clock"]
pub type ERTCDIV_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `CLKOUTDIV1` reader - Clock output division1"]
pub type CLKOUTDIV1_R = crate::FieldReader;
#[doc = "Field `CLKOUTDIV1` writer - Clock output division1"]
pub type CLKOUTDIV1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `CLKOUT_SEL1` reader - Clock output selection1"]
pub type CLKOUT_SEL1_R = crate::FieldReader;
#[doc = "Field `CLKOUT_SEL1` writer - Clock output selection1"]
pub type CLKOUT_SEL1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
impl R {
    #[doc = "Bits 0:1 - System clock select"]
    #[inline(always)]
    pub fn sclksel(&self) -> SCLKSEL_R {
        SCLKSEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - System Clock select Status"]
    #[inline(always)]
    pub fn sclksts(&self) -> SCLKSTS_R {
        SCLKSTS_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:7 - AHB division"]
    #[inline(always)]
    pub fn ahbdiv(&self) -> AHBDIV_R {
        AHBDIV_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 10:12 - APB1 division"]
    #[inline(always)]
    pub fn apb1div(&self) -> APB1DIV_R {
        APB1DIV_R::new(((self.bits >> 10) & 7) as u8)
    }
    #[doc = "Bits 13:15 - APB2 division"]
    #[inline(always)]
    pub fn apb2div(&self) -> APB2DIV_R {
        APB2DIV_R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:20 - HEXT division for ERTC clock"]
    #[inline(always)]
    pub fn ertcdiv(&self) -> ERTCDIV_R {
        ERTCDIV_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 27:29 - Clock output division1"]
    #[inline(always)]
    pub fn clkoutdiv1(&self) -> CLKOUTDIV1_R {
        CLKOUTDIV1_R::new(((self.bits >> 27) & 7) as u8)
    }
    #[doc = "Bits 30:31 - Clock output selection1"]
    #[inline(always)]
    pub fn clkout_sel1(&self) -> CLKOUT_SEL1_R {
        CLKOUT_SEL1_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - System clock select"]
    #[inline(always)]
    #[must_use]
    pub fn sclksel(&mut self) -> SCLKSEL_W<CFG_SPEC, 0> {
        SCLKSEL_W::new(self)
    }
    #[doc = "Bits 4:7 - AHB division"]
    #[inline(always)]
    #[must_use]
    pub fn ahbdiv(&mut self) -> AHBDIV_W<CFG_SPEC, 4> {
        AHBDIV_W::new(self)
    }
    #[doc = "Bits 10:12 - APB1 division"]
    #[inline(always)]
    #[must_use]
    pub fn apb1div(&mut self) -> APB1DIV_W<CFG_SPEC, 10> {
        APB1DIV_W::new(self)
    }
    #[doc = "Bits 13:15 - APB2 division"]
    #[inline(always)]
    #[must_use]
    pub fn apb2div(&mut self) -> APB2DIV_W<CFG_SPEC, 13> {
        APB2DIV_W::new(self)
    }
    #[doc = "Bits 16:20 - HEXT division for ERTC clock"]
    #[inline(always)]
    #[must_use]
    pub fn ertcdiv(&mut self) -> ERTCDIV_W<CFG_SPEC, 16> {
        ERTCDIV_W::new(self)
    }
    #[doc = "Bits 27:29 - Clock output division1"]
    #[inline(always)]
    #[must_use]
    pub fn clkoutdiv1(&mut self) -> CLKOUTDIV1_W<CFG_SPEC, 27> {
        CLKOUTDIV1_W::new(self)
    }
    #[doc = "Bits 30:31 - Clock output selection1"]
    #[inline(always)]
    #[must_use]
    pub fn clkout_sel1(&mut self) -> CLKOUT_SEL1_W<CFG_SPEC, 30> {
        CLKOUT_SEL1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Clock configuration register(CRM_CFG)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFG_SPEC;
impl crate::RegisterSpec for CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg::R`](R) reader structure"]
impl crate::Readable for CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cfg::W`](W) writer structure"]
impl crate::Writable for CFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFG to value 0"]
impl crate::Resettable for CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
