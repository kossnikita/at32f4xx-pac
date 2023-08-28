#[doc = "Register `MACMIIADDR` reader"]
pub type R = crate::R<MACMIIADDR_SPEC>;
#[doc = "Register `MACMIIADDR` writer"]
pub type W = crate::W<MACMIIADDR_SPEC>;
#[doc = "Field `MB` reader - MII busy"]
pub type MB_R = crate::BitReader;
#[doc = "Field `MB` writer - MII busy"]
pub type MB_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MW` reader - MII write"]
pub type MW_R = crate::BitReader;
#[doc = "Field `MW` writer - MII write"]
pub type MW_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CR` reader - Clock range"]
pub type CR_R = crate::FieldReader;
#[doc = "Field `CR` writer - Clock range"]
pub type CR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `MII` reader - MII register"]
pub type MII_R = crate::FieldReader;
#[doc = "Field `MII` writer - MII register"]
pub type MII_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `PA` reader - PHY address"]
pub type PA_R = crate::FieldReader;
#[doc = "Field `PA` writer - PHY address"]
pub type PA_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
impl R {
    #[doc = "Bit 0 - MII busy"]
    #[inline(always)]
    pub fn mb(&self) -> MB_R {
        MB_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - MII write"]
    #[inline(always)]
    pub fn mw(&self) -> MW_R {
        MW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:4 - Clock range"]
    #[inline(always)]
    pub fn cr(&self) -> CR_R {
        CR_R::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bits 6:10 - MII register"]
    #[inline(always)]
    pub fn mii(&self) -> MII_R {
        MII_R::new(((self.bits >> 6) & 0x1f) as u8)
    }
    #[doc = "Bits 11:15 - PHY address"]
    #[inline(always)]
    pub fn pa(&self) -> PA_R {
        PA_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - MII busy"]
    #[inline(always)]
    #[must_use]
    pub fn mb(&mut self) -> MB_W<MACMIIADDR_SPEC, 0> {
        MB_W::new(self)
    }
    #[doc = "Bit 1 - MII write"]
    #[inline(always)]
    #[must_use]
    pub fn mw(&mut self) -> MW_W<MACMIIADDR_SPEC, 1> {
        MW_W::new(self)
    }
    #[doc = "Bits 2:4 - Clock range"]
    #[inline(always)]
    #[must_use]
    pub fn cr(&mut self) -> CR_W<MACMIIADDR_SPEC, 2> {
        CR_W::new(self)
    }
    #[doc = "Bits 6:10 - MII register"]
    #[inline(always)]
    #[must_use]
    pub fn mii(&mut self) -> MII_W<MACMIIADDR_SPEC, 6> {
        MII_W::new(self)
    }
    #[doc = "Bits 11:15 - PHY address"]
    #[inline(always)]
    #[must_use]
    pub fn pa(&mut self) -> PA_W<MACMIIADDR_SPEC, 11> {
        PA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Ethernet MAC MII address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macmiiaddr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macmiiaddr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACMIIADDR_SPEC;
impl crate::RegisterSpec for MACMIIADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macmiiaddr::R`](R) reader structure"]
impl crate::Readable for MACMIIADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`macmiiaddr::W`](W) writer structure"]
impl crate::Writable for MACMIIADDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MACMIIADDR to value 0"]
impl crate::Resettable for MACMIIADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
