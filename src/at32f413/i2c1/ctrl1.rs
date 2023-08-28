#[doc = "Register `CTRL1` reader"]
pub type R = crate::R<CTRL1_SPEC>;
#[doc = "Register `CTRL1` writer"]
pub type W = crate::W<CTRL1_SPEC>;
#[doc = "Field `I2CEN` reader - Peripheral enable"]
pub type I2CEN_R = crate::BitReader;
#[doc = "Field `I2CEN` writer - Peripheral enable"]
pub type I2CEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PERMODE` reader - I2C peripheral mode"]
pub type PERMODE_R = crate::BitReader;
#[doc = "Field `PERMODE` writer - I2C peripheral mode"]
pub type PERMODE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SMBMODE` reader - SMBus device mode"]
pub type SMBMODE_R = crate::BitReader;
#[doc = "Field `SMBMODE` writer - SMBus device mode"]
pub type SMBMODE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ARPEN` reader - SMBus address resolution protocol enable"]
pub type ARPEN_R = crate::BitReader;
#[doc = "Field `ARPEN` writer - SMBus address resolution protocol enable"]
pub type ARPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PECEN` reader - PEC calculation enable"]
pub type PECEN_R = crate::BitReader;
#[doc = "Field `PECEN` writer - PEC calculation enable"]
pub type PECEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GCAEN` reader - General call address enable"]
pub type GCAEN_R = crate::BitReader;
#[doc = "Field `GCAEN` writer - General call address enable"]
pub type GCAEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `STRETCH` reader - Clock stretching mode"]
pub type STRETCH_R = crate::BitReader;
#[doc = "Field `STRETCH` writer - Clock stretching mode"]
pub type STRETCH_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GENSTART` reader - Start generation"]
pub type GENSTART_R = crate::BitReader;
#[doc = "Field `GENSTART` writer - Start generation"]
pub type GENSTART_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GENSTOP` reader - Stop generation"]
pub type GENSTOP_R = crate::BitReader;
#[doc = "Field `GENSTOP` writer - Stop generation"]
pub type GENSTOP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ACKEN` reader - Acknowledge enable"]
pub type ACKEN_R = crate::BitReader;
#[doc = "Field `ACKEN` writer - Acknowledge enable"]
pub type ACKEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MACKCTRL` reader - Master receiving mode acknowledge control"]
pub type MACKCTRL_R = crate::BitReader;
#[doc = "Field `MACKCTRL` writer - Master receiving mode acknowledge control"]
pub type MACKCTRL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PECTEN` reader - Request PEC transmission enable"]
pub type PECTEN_R = crate::BitReader;
#[doc = "Field `PECTEN` writer - Request PEC transmission enable"]
pub type PECTEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SMBALERT` reader - SMBus alert pin set"]
pub type SMBALERT_R = crate::BitReader;
#[doc = "Field `SMBALERT` writer - SMBus alert pin set"]
pub type SMBALERT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RESET` reader - I2C peripheral reset"]
pub type RESET_R = crate::BitReader;
#[doc = "Field `RESET` writer - I2C peripheral reset"]
pub type RESET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Peripheral enable"]
    #[inline(always)]
    pub fn i2cen(&self) -> I2CEN_R {
        I2CEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - I2C peripheral mode"]
    #[inline(always)]
    pub fn permode(&self) -> PERMODE_R {
        PERMODE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - SMBus device mode"]
    #[inline(always)]
    pub fn smbmode(&self) -> SMBMODE_R {
        SMBMODE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SMBus address resolution protocol enable"]
    #[inline(always)]
    pub fn arpen(&self) -> ARPEN_R {
        ARPEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PEC calculation enable"]
    #[inline(always)]
    pub fn pecen(&self) -> PECEN_R {
        PECEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - General call address enable"]
    #[inline(always)]
    pub fn gcaen(&self) -> GCAEN_R {
        GCAEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Clock stretching mode"]
    #[inline(always)]
    pub fn stretch(&self) -> STRETCH_R {
        STRETCH_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Start generation"]
    #[inline(always)]
    pub fn genstart(&self) -> GENSTART_R {
        GENSTART_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Stop generation"]
    #[inline(always)]
    pub fn genstop(&self) -> GENSTOP_R {
        GENSTOP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Acknowledge enable"]
    #[inline(always)]
    pub fn acken(&self) -> ACKEN_R {
        ACKEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Master receiving mode acknowledge control"]
    #[inline(always)]
    pub fn mackctrl(&self) -> MACKCTRL_R {
        MACKCTRL_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Request PEC transmission enable"]
    #[inline(always)]
    pub fn pecten(&self) -> PECTEN_R {
        PECTEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - SMBus alert pin set"]
    #[inline(always)]
    pub fn smbalert(&self) -> SMBALERT_R {
        SMBALERT_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - I2C peripheral reset"]
    #[inline(always)]
    pub fn reset(&self) -> RESET_R {
        RESET_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Peripheral enable"]
    #[inline(always)]
    #[must_use]
    pub fn i2cen(&mut self) -> I2CEN_W<CTRL1_SPEC, 0> {
        I2CEN_W::new(self)
    }
    #[doc = "Bit 1 - I2C peripheral mode"]
    #[inline(always)]
    #[must_use]
    pub fn permode(&mut self) -> PERMODE_W<CTRL1_SPEC, 1> {
        PERMODE_W::new(self)
    }
    #[doc = "Bit 3 - SMBus device mode"]
    #[inline(always)]
    #[must_use]
    pub fn smbmode(&mut self) -> SMBMODE_W<CTRL1_SPEC, 3> {
        SMBMODE_W::new(self)
    }
    #[doc = "Bit 4 - SMBus address resolution protocol enable"]
    #[inline(always)]
    #[must_use]
    pub fn arpen(&mut self) -> ARPEN_W<CTRL1_SPEC, 4> {
        ARPEN_W::new(self)
    }
    #[doc = "Bit 5 - PEC calculation enable"]
    #[inline(always)]
    #[must_use]
    pub fn pecen(&mut self) -> PECEN_W<CTRL1_SPEC, 5> {
        PECEN_W::new(self)
    }
    #[doc = "Bit 6 - General call address enable"]
    #[inline(always)]
    #[must_use]
    pub fn gcaen(&mut self) -> GCAEN_W<CTRL1_SPEC, 6> {
        GCAEN_W::new(self)
    }
    #[doc = "Bit 7 - Clock stretching mode"]
    #[inline(always)]
    #[must_use]
    pub fn stretch(&mut self) -> STRETCH_W<CTRL1_SPEC, 7> {
        STRETCH_W::new(self)
    }
    #[doc = "Bit 8 - Start generation"]
    #[inline(always)]
    #[must_use]
    pub fn genstart(&mut self) -> GENSTART_W<CTRL1_SPEC, 8> {
        GENSTART_W::new(self)
    }
    #[doc = "Bit 9 - Stop generation"]
    #[inline(always)]
    #[must_use]
    pub fn genstop(&mut self) -> GENSTOP_W<CTRL1_SPEC, 9> {
        GENSTOP_W::new(self)
    }
    #[doc = "Bit 10 - Acknowledge enable"]
    #[inline(always)]
    #[must_use]
    pub fn acken(&mut self) -> ACKEN_W<CTRL1_SPEC, 10> {
        ACKEN_W::new(self)
    }
    #[doc = "Bit 11 - Master receiving mode acknowledge control"]
    #[inline(always)]
    #[must_use]
    pub fn mackctrl(&mut self) -> MACKCTRL_W<CTRL1_SPEC, 11> {
        MACKCTRL_W::new(self)
    }
    #[doc = "Bit 12 - Request PEC transmission enable"]
    #[inline(always)]
    #[must_use]
    pub fn pecten(&mut self) -> PECTEN_W<CTRL1_SPEC, 12> {
        PECTEN_W::new(self)
    }
    #[doc = "Bit 13 - SMBus alert pin set"]
    #[inline(always)]
    #[must_use]
    pub fn smbalert(&mut self) -> SMBALERT_W<CTRL1_SPEC, 13> {
        SMBALERT_W::new(self)
    }
    #[doc = "Bit 15 - I2C peripheral reset"]
    #[inline(always)]
    #[must_use]
    pub fn reset(&mut self) -> RESET_W<CTRL1_SPEC, 15> {
        RESET_W::new(self)
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
