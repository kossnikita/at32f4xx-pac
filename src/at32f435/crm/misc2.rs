#[doc = "Register `MISC2` reader"]
pub type R = crate::R<MISC2_SPEC>;
#[doc = "Register `MISC2` writer"]
pub type W = crate::W<MISC2_SPEC>;
#[doc = "Field `AUTO_STEP_EN` reader - AUTO_STEP_EN"]
pub type AUTO_STEP_EN_R = crate::FieldReader;
#[doc = "Field `AUTO_STEP_EN` writer - AUTO_STEP_EN"]
pub type AUTO_STEP_EN_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CLK_TO_TMR` reader - Clock output internal connect to timer10"]
pub type CLK_TO_TMR_R = crate::BitReader;
#[doc = "Field `CLK_TO_TMR` writer - Clock output internal connect to timer10"]
pub type CLK_TO_TMR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBDIV` reader - USB division"]
pub type USBDIV_R = crate::FieldReader;
#[doc = "Field `USBDIV` writer - USB division"]
pub type USBDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 4:5 - AUTO_STEP_EN"]
    #[inline(always)]
    pub fn auto_step_en(&self) -> AUTO_STEP_EN_R {
        AUTO_STEP_EN_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 8 - Clock output internal connect to timer10"]
    #[inline(always)]
    pub fn clk_to_tmr(&self) -> CLK_TO_TMR_R {
        CLK_TO_TMR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 12:15 - USB division"]
    #[inline(always)]
    pub fn usbdiv(&self) -> USBDIV_R {
        USBDIV_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MISC2")
            .field("auto_step_en", &self.auto_step_en())
            .field("clk_to_tmr", &self.clk_to_tmr())
            .field("usbdiv", &self.usbdiv())
            .finish()
    }
}
impl W {
    #[doc = "Bits 4:5 - AUTO_STEP_EN"]
    #[inline(always)]
    #[must_use]
    pub fn auto_step_en(&mut self) -> AUTO_STEP_EN_W<MISC2_SPEC> {
        AUTO_STEP_EN_W::new(self, 4)
    }
    #[doc = "Bit 8 - Clock output internal connect to timer10"]
    #[inline(always)]
    #[must_use]
    pub fn clk_to_tmr(&mut self) -> CLK_TO_TMR_W<MISC2_SPEC> {
        CLK_TO_TMR_W::new(self, 8)
    }
    #[doc = "Bits 12:15 - USB division"]
    #[inline(always)]
    #[must_use]
    pub fn usbdiv(&mut self) -> USBDIV_W<MISC2_SPEC> {
        USBDIV_W::new(self, 12)
    }
}
#[doc = "Miscellaneous register2\n\nYou can [`read`](crate::Reg::read) this register and get [`misc2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`misc2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MISC2_SPEC;
impl crate::RegisterSpec for MISC2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`misc2::R`](R) reader structure"]
impl crate::Readable for MISC2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`misc2::W`](W) writer structure"]
impl crate::Writable for MISC2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MISC2 to value 0x0d"]
impl crate::Resettable for MISC2_SPEC {
    const RESET_VALUE: u32 = 0x0d;
}
